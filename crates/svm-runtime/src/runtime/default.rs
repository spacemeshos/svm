use std::{convert::TryFrom, ffi::c_void, fmt};

use log::{debug, error, info};

use crate::{
    buffer::BufferRef,
    ctx::SvmCtx,
    error::{DeployTemplateError, ExecAppError, SpawnAppError},
    helpers,
    helpers::DataWrapper,
    runtime::Receipt,
    settings::AppSettings,
    traits::{Runtime, StorageBuilderFn},
    value::Value,
};

use svm_app::{
    traits::{Env, EnvTypes},
    types::{
        AppAddr, AppTemplate, AppTransaction, AuthorAddr, CreatorAddr, HostCtx, SpawnApp,
        TemplateAddr, WasmValue,
    },
};
use svm_common::{Address, State};
use svm_storage::AppStorage;

use wasmer_runtime::Value as WasmerValue;
use wasmer_runtime_core::{
    export::Export,
    import::{ImportObject, Namespace},
};

/// Default `Runtime` implementation based on `wasmer`.
pub struct DefaultRuntime<ENV> {
    /// The runtime environment. Used mainly for managing app persistence.
    pub env: ENV,

    /// A raw pointer to host (a.k.a the `Full-Node` in the realm of Blockchain).
    pub host: *mut c_void,

    /// External `wasmer` imports (living inside the host) to be consumed by the app.
    pub imports: Vec<(String, String, Export)>,

    /// Determined by the app `Address` and `State` (app state) and app storage settings,
    /// builds a `AppStorage` instance.
    pub storage_builder: Box<StorageBuilderFn>,
}

impl<TY, ENV> Runtime for DefaultRuntime<ENV>
where
    TY: EnvTypes,
    ENV: Env<Types = TY>,
{
    fn deploy_template(
        &mut self,
        author: &AuthorAddr,
        host_ctx: HostCtx,
        bytes: &[u8],
    ) -> Result<TemplateAddr, DeployTemplateError> {
        info!("runtime `deploy_template`");

        let template = self.parse_deploy_template(bytes)?;
        self.install_template(&template, author, &host_ctx)
    }

    fn spawn_app(
        &mut self,
        creator: &CreatorAddr,
        host_ctx: HostCtx,
        bytes: &[u8],
    ) -> Result<(AppAddr, State), SpawnAppError> {
        info!("runtime `spawn_app`");

        let spawn = self.parse_spawn_app(bytes)?;
        let app_addr = self.install_app(&spawn, creator, &host_ctx)?;
        let state = self.call_ctor(creator, spawn, &app_addr, host_ctx)?;

        Ok((app_addr, state))
    }

    fn parse_exec_app(&self, bytes: &[u8]) -> Result<AppTransaction, ExecAppError> {
        self.env
            .parse_exec_app(bytes)
            .or_else(|e| Err(ExecAppError::ParseFailed(e)))
    }

    fn exec_app(
        &self,
        tx: AppTransaction,
        state: State,
        host_ctx: HostCtx,
        dry_run: bool,
    ) -> Result<Receipt, ExecAppError> {
        let is_ctor = false;

        self.inner_exec_app(tx, state, host_ctx, is_ctor, dry_run)
    }
}

impl<TY, ENV> DefaultRuntime<ENV>
where
    TY: EnvTypes,
    ENV: Env<Types = TY>,
{
    /// Initializes a new `DefaultRuntime` instance.
    pub fn new(
        host: *mut c_void,
        env: ENV,
        imports: Vec<(String, String, Export)>,
        storage_builder: Box<StorageBuilderFn>,
    ) -> Self {
        Self {
            env,
            host,
            imports,
            storage_builder,
        }
    }

    /// Initialize a new `AppStorage` and returns it.
    /// This method is of `pub` visibility since it's also helpful for tests that want to
    /// observe that app storage data.
    pub fn open_app_storage(
        &self,
        addr: &AppAddr,
        state: &State,
        settings: &AppSettings,
    ) -> AppStorage {
        let sb = &self.storage_builder;
        sb(addr, state, settings)
    }

    fn call_ctor(
        &mut self,
        creator: &CreatorAddr,
        spawn_app: SpawnApp,
        app_addr: &AppAddr,
        host_ctx: HostCtx,
    ) -> Result<State, SpawnAppError> {
        let ctor = self.build_ctor_call(creator, spawn_app, &app_addr);
        let is_ctor = true;
        let dry_run = false;

        match self.inner_exec_app(ctor, State::empty(), host_ctx, is_ctor, dry_run) {
            Ok(receipt) => {
                // TODO:
                // handle `receipt.success = false`

                let new_state = receipt.new_state.unwrap();
                Ok(new_state)
            }
            Err(..) => {
                todo!()
                // return `SpawnAppError` of `ctor failed`
            }
        }
    }

    fn parse_deploy_template(&self, bytes: &[u8]) -> Result<AppTemplate, DeployTemplateError> {
        self.env
            .parse_deploy_template(bytes)
            .or_else(|e| Err(DeployTemplateError::ParseFailed(e)))
    }

    fn install_template(
        &mut self,
        template: &AppTemplate,
        author: &AuthorAddr,
        host_ctx: &HostCtx,
    ) -> Result<TemplateAddr, DeployTemplateError> {
        self.env
            .store_template(template, author, host_ctx)
            .or_else(|e| Err(DeployTemplateError::StoreFailed(e)))
    }

    fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, SpawnAppError> {
        self.env
            .parse_spawn_app(bytes)
            .or_else(|e| Err(SpawnAppError::ParseFailed(e)))
    }

    fn install_app(
        &mut self,
        spawn: &SpawnApp,
        creator: &CreatorAddr,
        host_ctx: &HostCtx,
    ) -> Result<AppAddr, SpawnAppError> {
        self.env
            .store_app(spawn, creator, host_ctx)
            .or_else({ |e| Err(SpawnAppError::StoreFailed(e)) })
    }

    fn build_ctor_call(
        &self,
        creator: &CreatorAddr,
        spawn: SpawnApp,
        app_addr: &AppAddr,
    ) -> AppTransaction {
        AppTransaction {
            version: 0,
            app: app_addr.clone(),
            func_idx: spawn.ctor_idx,
            func_args: spawn.ctor_args,
            func_buf: spawn.ctor_buf,
        }
    }

    fn inner_exec_app(
        &self,
        tx: AppTransaction,
        state: State,
        host_ctx: HostCtx,
        is_ctor: bool,
        dry_run: bool,
    ) -> Result<Receipt, ExecAppError> {
        info!("runtime `exec_app`");

        let (template, template_addr, _author, _creator) = self.load_template(&tx)?;

        let settings = AppSettings {
            page_count: template.page_count,
        };

        let mut import_object = self.import_object_create(&tx.app, &state, host_ctx, &settings);
        self.import_object_extend(&mut import_object);

        let result = self.do_exec_app(
            &tx,
            &template,
            &template_addr,
            &import_object,
            is_ctor,
            dry_run,
        );
        let receipt = self.make_receipt(result);

        info!("receipt: {:?}", receipt);

        Ok(receipt)
    }

    fn do_exec_app(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &TemplateAddr,
        import_object: &ImportObject,
        is_ctor: bool,
        dry_run: bool,
    ) -> Result<(Option<State>, Vec<Value>), ExecAppError> {
        let module = self.compile_template(tx, &template, &template_addr)?;
        let mut instance = self.instantiate(tx, template_addr, &module, import_object)?;

        self.init_instance_buffer(&tx.func_buf, &mut instance);

        let args = self.prepare_args_and_memory(tx);

        let func = match self.get_exported_func(tx, template_addr, &instance) {
            Err(ExecAppError::FuncNotFound { .. }) if is_ctor == true => {
                // Since an app `ctor` is optional, in case it has no explicit `ctor`
                // we **don't** consider it as an error.
                let empty_state = State::empty();

                return Ok((Some(empty_state), Vec::new()));
            }
            Err(e) => return Err(e),
            Ok(func) => func,
        };

        match func.call(&args) {
            Err(e) => Err(ExecAppError::ExecFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_idx: tx.func_idx,
                func_args: self.vec_to_str(&tx.func_args),
                reason: e.to_string(),
            }),
            Ok(returns) => {
                let new_state = if dry_run {
                    None
                } else {
                    let storage = self.instance_storage_mut(&mut instance);
                    let new_state = storage.commit();

                    Some(new_state)
                };

                let returns = self.cast_wasmer_func_returns(tx, template_addr, returns)?;

                Ok((new_state, returns))
            }
        }
    }

    fn make_receipt(&self, result: Result<(Option<State>, Vec<Value>), ExecAppError>) -> Receipt {
        match result {
            Err(e) => Receipt {
                success: false,
                error: Some(e),
                returns: None,
                new_state: None,
            },
            Ok((new_state, returns)) => Receipt {
                success: true,
                error: None,
                returns: Some(returns),
                new_state,
            },
        }
    }

    fn init_instance_buffer(&self, func_buf: &Vec<u8>, instance: &mut wasmer_runtime::Instance) {
        const ARGS_BUF_ID: u32 = 0;

        let ctx = instance.context_mut();

        helpers::buffer_create(ctx.data, ARGS_BUF_ID, func_buf.len() as u32);

        match helpers::wasmer_data_buffer(ctx.data, ARGS_BUF_ID).unwrap() {
            BufferRef::Mutable(.., buf) => {
                buf.write(&func_buf[..]);
            }
            _ => unreachable!(),
        };

        helpers::buffer_freeze(ctx.data, ARGS_BUF_ID);
    }

    fn cast_wasmer_func_returns(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        returns: Vec<WasmerValue>,
    ) -> Result<Vec<Value>, ExecAppError> {
        let mut values = Vec::new();

        for ret in returns.iter() {
            match Value::try_from(ret) {
                Err(e) => {
                    return Err(ExecAppError::InvalidReturnValue {
                        app_addr: tx.app.clone(),
                        template_addr: template_addr.clone(),
                        func_idx: tx.func_idx,
                        func_args: self.vec_to_str(&tx.func_args),
                        func_rets: self.vec_to_str(&returns),
                        reason: e.to_string(),
                    })
                }
                Ok(v) => values.push(v),
            }
        }

        Ok(values)
    }

    fn instantiate(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        module: &wasmer_runtime::Module,
        import_object: &ImportObject,
    ) -> Result<wasmer_runtime::Instance, ExecAppError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        module.instantiate(import_object).or_else(|e| {
            Err(ExecAppError::InstantiationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                reason: e.to_string(),
            })
        })
    }

    fn get_exported_func<'a>(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        instance: &'a wasmer_runtime::Instance,
    ) -> Result<wasmer_runtime::DynFunc<'a>, ExecAppError> {
        let func_idx = self.derive_func_index(instance, tx);

        instance.dyn_func_with_index(func_idx).or_else(|_e| {
            error!("Exported function: `{}` not found", func_idx);

            Err(ExecAppError::FuncNotFound {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_idx: *&tx.func_idx,
            })
        })
    }

    fn derive_func_index(&self, instance: &wasmer_runtime::Instance, tx: &AppTransaction) -> usize {
        let rel_func_index = *&tx.func_idx as usize;
        let imported_funcs = instance.module.info.imported_functions.len();
        let func_index = rel_func_index + imported_funcs;

        func_index
    }

    fn prepare_args_and_memory(&self, tx: &AppTransaction) -> Vec<wasmer_runtime::Value> {
        debug!("runtime `prepare_args_and_memory`");

        let mut wasmer_args = Vec::with_capacity(tx.func_args.len());

        for arg in tx.func_args.iter() {
            let wasmer_arg = match arg {
                WasmValue::I32(v) => WasmerValue::I32(*v as i32),
                WasmValue::I64(v) => WasmerValue::I64(*v as i64),
            };

            wasmer_args.push(wasmer_arg);
        }

        debug!("wasmer args={:?}", wasmer_args);

        wasmer_args
    }

    #[inline]
    fn instance_storage_mut(
        &self,
        instance: &mut wasmer_runtime::Instance,
    ) -> &mut svm_storage::AppStorage {
        let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();
        helpers::wasmer_data_app_storage(wasmer_ctx.data)
    }

    fn import_object_create(
        &self,
        addr: &AppAddr,
        state: &State,
        host_ctx: HostCtx,
        settings: &AppSettings,
    ) -> ImportObject {
        debug!(
            "runtime `import_object_create` address={:?}, state={:?}, settings={:?}",
            addr, state, settings
        );

        let storage = self.open_app_storage(addr, state, settings);
        let host_ctx = svm_common::into_raw(host_ctx);

        let svm_ctx = SvmCtx::new(
            DataWrapper::new(self.host),
            DataWrapper::new(host_ctx),
            storage,
        );
        let svm_ctx = Box::leak(Box::new(svm_ctx));

        let state_creator = move || {
            let data: *mut c_void = svm_ctx as *const SvmCtx as *mut SvmCtx as _;

            let dtor: fn(*mut c_void) = |ctx_data| {
                let ctx_ptr = ctx_data as *mut SvmCtx;

                // triggers memory releasing
                unsafe {
                    let _ = Box::from_raw(ctx_ptr);
                }
            };

            (data, dtor)
        };

        ImportObject::new_with_data(state_creator)
    }

    fn import_object_extend(&self, import_object: &mut ImportObject) {
        // TODO: validate that `self.imports` don't use `svm` as import namespaces.

        import_object.extend(self.imports.clone());

        let mut ns = Namespace::new();
        crate::vmcalls::insert_vmcalls(&mut ns);

        import_object.register("svm", ns);
    }

    fn load_template(
        &self,
        tx: &AppTransaction,
    ) -> Result<(AppTemplate, TemplateAddr, AuthorAddr, CreatorAddr), ExecAppError> {
        info!("runtime `load_template`");

        self.env
            .load_template_by_app(&tx.app)
            .ok_or_else(|| ExecAppError::AppNotFound {
                app_addr: tx.app.clone(),
            })
    }

    fn compile_template(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &TemplateAddr,
    ) -> Result<wasmer_runtime::Module, ExecAppError> {
        info!("runtime `compile_template` (template={:?})", template_addr);

        svm_compiler::compile_program(&template.code).or_else(|e| {
            error!("module compilation failed (template={:?})", template_addr);

            Err(ExecAppError::CompilationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                reason: e.to_string(),
            })
        })
    }

    fn vec_to_str<T: fmt::Debug>(&self, items: &Vec<T>) -> String {
        let mut buf = String::new();

        for (i, arg) in items.iter().enumerate() {
            if i != 0 {
                buf.push_str(", ");
            }
            buf.push_str(&format!("{:?}, ", arg));
        }

        buf
    }
}

impl<ENV> Drop for DefaultRuntime<ENV> {
    fn drop(&mut self) {
        info!("dropping Runtime...");
    }
}
