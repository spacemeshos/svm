use std::convert::TryFrom;
use std::ffi::c_void;
use std::fmt;

use log::{debug, error, info};

use crate::{
    buffer::{Buffer, BufferMut},
    ctx::SvmCtx,
    error::{DeployTemplateError, ExecAppError, SpawnAppError},
    helpers,
    helpers::DataWrapper,
    host_ctx::HostCtx,
    runtime::Receipt,
    settings::AppSettings,
    traits::{Runtime, StorageBuilderFn},
    value::Value,
};

use svm_app::{
    traits::{Env, EnvTypes},
    types::{App, AppTemplate, AppTransaction, BufferSlice, WasmArgValue},
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
        author: &Address,
        _host_ctx: HostCtx,
        bytes: &[u8],
    ) -> Result<Address, DeployTemplateError> {
        info!("runtime `deploy_template`");

        let template = self.parse_template(author, bytes)?;
        self.install_template(&template)
    }

    fn spawn_app(
        &mut self,
        creator: &Address,
        host_ctx: HostCtx,
        bytes: &[u8],
    ) -> Result<(Address, State), SpawnAppError> {
        info!("runtime `spawn_app`");

        let (app, ctor_buf_slices) = self.parse_app(creator, bytes)?;
        let app_addr = self.install_app(&app)?;
        let state = self.call_ctor(creator, &app, &app_addr, ctor_buf_slices, host_ctx)?;

        Ok((app_addr, state))
    }

    fn parse_exec_app(
        &self,
        sender: &Address,
        bytes: &[u8],
    ) -> Result<AppTransaction, ExecAppError> {
        match self.env.parse_app_tx(bytes, sender) {
            Ok(tx) => Ok(tx),
            Err(e) => Err(ExecAppError::ParseFailed(e)),
        }
    }

    fn exec_app(
        &self,
        tx: AppTransaction,
        state: State,
        host_ctx: HostCtx,
    ) -> Result<Receipt, ExecAppError> {
        let is_ctor = false;
        self.inner_exec_app(tx, state, host_ctx, is_ctor)
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
        addr: &Address,
        state: &State,
        settings: &AppSettings,
    ) -> AppStorage {
        let sb = &self.storage_builder;
        sb(addr, state, settings)
    }

    fn call_ctor(
        &mut self,
        creator: &Address,
        app: &App,
        app_addr: &Address,
        ctor_buf_slices: Vec<BufferSlice>,
        host_ctx: HostCtx,
    ) -> Result<State, SpawnAppError> {
        let ctor = self.build_ctor_call(creator, &app, &app_addr, ctor_buf_slices);
        let is_ctor = true;

        match self.inner_exec_app(ctor, State::empty(), host_ctx, is_ctor) {
            Ok(receipt) => {
                let new_state = receipt.new_state.unwrap();
                Ok(new_state)
            }
            Err(..) => {
                todo!()
                // return `SpawnAppError` of `ctor failed`
            }
        }
    }

    fn parse_template(
        &self,
        author: &Address,
        bytes: &[u8],
    ) -> Result<AppTemplate, DeployTemplateError> {
        match self.env.parse_template(bytes, author) {
            Ok(template) => Ok(template),
            Err(e) => Err(DeployTemplateError::ParseFailed(e)),
        }
    }

    fn install_template(&mut self, template: &AppTemplate) -> Result<Address, DeployTemplateError> {
        match self.env.store_template(template) {
            Ok(addr) => Ok(addr),
            Err(e) => Err(DeployTemplateError::StoreFailed(e)),
        }
    }

    fn parse_app(
        &self,
        creator: &Address,
        bytes: &[u8],
    ) -> Result<(App, Vec<BufferSlice>), SpawnAppError> {
        match self.env.parse_app(bytes, creator) {
            Ok((app, ctor_buf_args)) => Ok((app, ctor_buf_args)),
            Err(e) => Err(SpawnAppError::ParseFailed(e)),
        }
    }

    fn install_app(&mut self, app: &App) -> Result<Address, SpawnAppError> {
        match self.env.store_app(app) {
            Ok(app_addr) => Ok(app_addr),
            Err(e) => Err(SpawnAppError::StoreFailed(e)),
        }
    }

    fn build_ctor_call(
        &self,
        creator: &Address,
        app: &App,
        app_addr: &Address,
        ctor_buf_slices: Vec<BufferSlice>,
    ) -> AppTransaction {
        AppTransaction {
            app: app_addr.clone(),
            sender: creator.clone(),
            func_name: "ctor".to_string(),
            func_args: vec![],
            func_args_buf: ctor_buf_slices,
        }
    }

    fn inner_exec_app(
        &self,
        tx: AppTransaction,
        state: State,
        host_ctx: HostCtx,
        is_ctor: bool,
    ) -> Result<Receipt, ExecAppError> {
        info!("runtime `exec_app`");

        let (template, template_addr) = self.load_template(&tx)?;

        let settings = AppSettings {
            pages_count: template.pages_count,
        };

        let mut import_object = self.import_object_create(&tx.app, &state, host_ctx, &settings);
        self.import_object_extend(&mut import_object);

        let receipt =
            match self.do_exec_app(&tx, &template, &template_addr, &import_object, is_ctor) {
                Err(e) => Receipt {
                    success: false,
                    error: Some(e),
                    returns: None,
                    new_state: None,
                },
                Ok((state, returns)) => Receipt {
                    success: true,
                    error: None,
                    returns: Some(returns),
                    new_state: Some(state),
                },
            };

        info!("receipt: {:?}", receipt);

        Ok(receipt)
    }

    fn do_exec_app(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &Address,
        import_object: &ImportObject,
        is_ctor: bool,
    ) -> Result<(State, Vec<Value>), ExecAppError> {
        let module = self.compile_template(tx, &template, &template_addr)?;
        let mut instance = self.instantiate(tx, template_addr, &module, import_object)?;

        self.init_instance_buffers(&tx.func_args_buf, &mut instance);
        let args = self.prepare_args_and_memory(tx, &mut instance);

        let func = match self.get_exported_func(tx, template_addr, &instance) {
            Err(ExecAppError::FuncNotFound { .. }) if is_ctor == true => {
                // Since an app `ctor` is optional, in case it has no explicit `ctor`
                // we **don't** consider is as an error.
                return Ok((State::empty(), Vec::new()));
            }
            Err(e) => return Err(e),
            Ok(func) => func,
        };

        match func.call(&args) {
            Err(e) => Err(ExecAppError::ExecFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_name: tx.func_name.clone(),
                func_args: self.vec_to_str(&tx.func_args),
                reason: e.to_string(),
            }),
            Ok(returns) => {
                let storage = self.instance_storage_mut(&mut instance);
                let state = storage.commit();
                let returns = self.cast_wasmer_func_returns(tx, template_addr, returns)?;

                Ok((state, returns))
            }
        }
    }

    fn init_instance_buffers(
        &self,
        func_args_buf: &Vec<BufferSlice>,
        instance: &mut wasmer_runtime::Instance,
    ) {
        // TODO:
        // * create buffers out of `func_args_buf`
        // * pass the buffer to `instance`
    }

    fn cast_wasmer_func_returns(
        &self,
        tx: &AppTransaction,
        template_addr: &Address,
        returns: Vec<WasmerValue>,
    ) -> Result<Vec<Value>, ExecAppError> {
        let mut values = Vec::new();

        for ret in returns.iter() {
            match Value::try_from(ret) {
                Err(e) => {
                    return Err(ExecAppError::InvalidReturnValue {
                        app_addr: tx.app.clone(),
                        template_addr: template_addr.clone(),
                        func_name: tx.func_name.clone(),
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
        template_addr: &Address,
        module: &wasmer_runtime::Module,
        import_object: &ImportObject,
    ) -> Result<wasmer_runtime::Instance, ExecAppError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        let instantiate = module.instantiate(import_object);

        match instantiate {
            Err(e) => Err(ExecAppError::InstantiationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                reason: e.to_string(),
            }),
            Ok(instance) => Ok(instance),
        }
    }

    fn get_exported_func<'a>(
        &self,
        tx: &AppTransaction,
        template_addr: &Address,
        instance: &'a wasmer_runtime::Instance,
    ) -> Result<wasmer_runtime::DynFunc<'a>, ExecAppError> {
        let func_name = &tx.func_name;
        let func = instance.dyn_func(func_name);

        match func {
            Err(_e) => {
                error!("Exported function: `{}` not found", func_name);

                Err(ExecAppError::FuncNotFound {
                    app_addr: tx.app.clone(),
                    template_addr: template_addr.clone(),
                    func_name: func_name.to_string(),
                })
            }
            Ok(func) => {
                info!("Found exported function `{}`", func_name);

                Ok(func)
            }
        }
    }

    fn prepare_args_and_memory(
        &self,
        tx: &AppTransaction,
        instance: &mut wasmer_runtime::Instance,
    ) -> Vec<wasmer_runtime::Value> {
        debug!("runtime `prepare_args_and_memory`");

        let memory = instance.context_mut().memory(0);
        let mut mem_offset = 0;

        let mut wasmer_args = Vec::with_capacity(tx.func_args.len());

        for arg in tx.func_args.iter() {
            let wasmer_arg = match arg {
                WasmArgValue::I32(v) => WasmerValue::I32(*v as i32),
                WasmArgValue::I64(v) => WasmerValue::I64(*v as i64),
            };

            wasmer_args.push(wasmer_arg);
        }

        debug!("wasmer args={:?}", wasmer_args);

        wasmer_args
    }

    #[inline(always)]
    fn instance_storage_mut(
        &self,
        instance: &mut wasmer_runtime::Instance,
    ) -> &mut svm_storage::AppStorage {
        let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();
        helpers::wasmer_data_app_storage(wasmer_ctx.data)
    }

    fn import_object_create(
        &self,
        addr: &Address,
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
                unsafe { Box::from_raw(ctx_ptr) };
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

    fn load_template(&self, tx: &AppTransaction) -> Result<(AppTemplate, Address), ExecAppError> {
        info!("runtime `load_template`");

        match self.env.load_template_by_app(&tx.app) {
            Some(res) => Ok(res),
            None => Err(ExecAppError::AppNotFound {
                app_addr: tx.app.clone(),
            }),
        }
    }

    fn compile_template(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &Address,
    ) -> Result<wasmer_runtime::Module, ExecAppError> {
        info!("runtime `compile_template` (template={:?})", template_addr);

        match svm_compiler::compile_program(&template.code) {
            Ok(module) => {
                debug!("module compile succeeded (template={:?})", template_addr);

                Ok(module)
            }
            Err(e) => {
                error!("module compilation failed (template={:?})", template_addr);

                Err(ExecAppError::CompilationFailed {
                    app_addr: tx.app.clone(),
                    template_addr: template_addr.clone(),
                    reason: e.to_string(),
                })
            }
        }
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
