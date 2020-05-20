use std::{
    ffi::c_void,
    fmt,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use log::{debug, error, info};

use crate::{
    buffer::BufferRef,
    ctx::SvmCtx,
    error::{ExecAppError, SpawnAppError, ValidateError},
    gas::{GasEstimator, MaybeGas, OOGError},
    helpers::{self, DataWrapper},
    receipt::{make_spawn_app_receipt, ExecReceipt, SpawnAppReceipt, TemplateReceipt},
    runtime::Runtime,
    settings::AppSettings,
    storage::StorageBuilderFn,
};

use svm_app::{
    error::ParseError,
    traits::{Env, EnvTypes},
    types::{
        AppAddr, AppTemplate, AppTransaction, AuthorAddr, CreatorAddr, HostCtx, SpawnApp,
        TemplateAddr, WasmValue,
    },
};
use svm_common::State;
use svm_gas::Gas;
use svm_storage::app::AppStorage;

use wasmer_runtime::Value as WasmerValue;
use wasmer_runtime_core::{
    export::Export,
    import::{ImportObject, Namespace},
};

/// Default `Runtime` implementation based on `wasmer`.
pub struct DefaultRuntime<ENV, GE> {
    /// The runtime environment. Used mainly for managing app persistence.
    pub env: ENV,

    /// A raw pointer to host (a.k.a the `Full-Node` in the realm of Blockchain).
    pub host: *mut c_void,

    /// kv-store (rocksdb/leveldb/similar) path where it stores SVM data (templates, app, app-storage).
    pub kv_path: PathBuf,

    /// External `wasmer` imports (living inside the host) to be consumed by the app.
    pub imports: Vec<(String, String, Export)>,

    /// builds a `AppStorage` instance.
    pub storage_builder: Box<StorageBuilderFn>,

    phantom: PhantomData<GE>,
}

impl<TY, ENV, GE> Runtime for DefaultRuntime<ENV, GE>
where
    TY: EnvTypes,
    ENV: Env<Types = TY>,
    GE: GasEstimator,
{
    fn validate_template(&self, bytes: &[u8]) -> Result<(), ValidateError> {
        let template = self.parse_deploy_template(bytes)?;
        let wasm = &template.code[..];

        svm_gas::validate_code(wasm).map_err(|e| e.into())
    }

    fn validate_app(&self, bytes: &[u8]) -> Result<(), ValidateError> {
        self.parse_spawn_app(bytes)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn validate_tx(&self, bytes: &[u8]) -> Result<AppAddr, ValidateError> {
        self.env
            .parse_exec_app(bytes)
            .map(|tx| tx.app)
            .map_err(|e| e.into())
    }

    fn estimate_deploy_template(&self, bytes: &[u8]) -> Result<Gas, ValidateError> {
        self.validate_template(bytes)?;

        todo!()
    }

    fn estimate_spawn_app(&self, bytes: &[u8]) -> Result<Gas, ValidateError> {
        self.validate_app(bytes)?;

        todo!()
    }

    fn estimate_exec_app(&self, bytes: &[u8]) -> Result<Gas, ValidateError> {
        self.validate_tx(bytes)?;

        todo!()
    }

    fn deploy_template(
        &mut self,
        bytes: &[u8],
        author: &AuthorAddr,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
    ) -> TemplateReceipt {
        info!("runtime `deploy_template`");

        let template = self.parse_deploy_template(bytes).unwrap();
        let install_gas = self.compute_install_template_gas(bytes, &template);

        if gas_limit >= install_gas {
            let gas_used = MaybeGas::with(install_gas);
            let gas_left = gas_limit;

            self.install_template(&template, author, host_ctx, gas_used, gas_left)
        } else {
            TemplateReceipt::new_oog()
        }
    }

    fn spawn_app(
        &mut self,
        bytes: &[u8],
        creator: &CreatorAddr,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
    ) -> SpawnAppReceipt {
        info!("runtime `spawn_app`");

        let spawn = self.parse_spawn_app(bytes).unwrap();
        let install_gas = self.compute_install_app_gas(bytes, &spawn);

        let gas_left = gas_limit - install_gas;

        match gas_left {
            Err(..) => SpawnAppReceipt::new_oog(),
            Ok(gas_left) => match self.install_app(&spawn, creator, &host_ctx) {
                Ok(addr) => {
                    let gas_used = install_gas.into();

                    self.call_ctor(creator, spawn, &addr, host_ctx, gas_used, gas_left)
                }
                Err(e) => e.into(),
            },
        }
    }

    fn exec_app(
        &self,
        bytes: &[u8],
        state: &State,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
    ) -> ExecReceipt {
        let tx = self.parse_exec_app(bytes).unwrap();
        let gas_used = MaybeGas::with(0);

        self._exec_app(&tx, state, host_ctx, gas_used, gas_limit)
    }
}

impl<TY, ENV, GE> DefaultRuntime<ENV, GE>
where
    TY: EnvTypes,
    ENV: Env<Types = TY>,
    GE: GasEstimator,
{
    /// Initializes a new `DefaultRuntime` instance.
    pub fn new<P: AsRef<Path>>(
        host: *mut c_void,
        env: ENV,
        kv_path: P,
        imports: Vec<(String, String, Export)>,
        storage_builder: Box<StorageBuilderFn>,
    ) -> Self {
        Self::ensure_not_svm_ns(&imports[..]);

        Self {
            env,
            host,
            kv_path: kv_path.as_ref().to_path_buf(),
            imports,
            storage_builder,
            phantom: PhantomData::<GE>,
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
        (self.storage_builder)(addr, state, settings)
    }

    fn call_ctor(
        &mut self,
        creator: &CreatorAddr,
        spawn: SpawnApp,
        app_addr: &AppAddr,
        host_ctx: HostCtx,
        gas_used: MaybeGas,
        gas_left: MaybeGas,
    ) -> SpawnAppReceipt {
        let ctor = self.build_ctor_call(creator, spawn, app_addr);

        let ctor_receipt = self._exec_app(&ctor, &State::empty(), host_ctx, gas_used, gas_left);

        make_spawn_app_receipt(ctor_receipt, app_addr)
    }

    fn install_template(
        &mut self,
        template: &AppTemplate,
        author: &AuthorAddr,
        host_ctx: HostCtx,
        gas_used: MaybeGas,
        _gas_left: MaybeGas,
    ) -> TemplateReceipt {
        match self.env.store_template(template, author, &host_ctx) {
            Ok(addr) => TemplateReceipt::new(addr, gas_used),
            Err(..) => panic!("Store failed"),
        }
    }

    fn install_app(
        &mut self,
        spawn: &SpawnApp,
        creator: &CreatorAddr,
        host_ctx: &HostCtx,
    ) -> Result<AppAddr, SpawnAppError> {
        self.env
            .store_app(spawn, creator, host_ctx)
            .or_else(|e| Err(SpawnAppError::StoreFailed(e)))
    }

    fn build_ctor_call(
        &self,
        _creator: &CreatorAddr,
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

    fn _exec_app(
        &self,
        tx: &AppTransaction,
        state: &State,
        host_ctx: HostCtx,
        _gas_used: MaybeGas,
        gas_left: MaybeGas,
    ) -> ExecReceipt {
        info!("runtime `exec_app`");

        match self.load_template(&tx) {
            Err(e) => e.into(),
            Ok((template, template_addr, _author, _creator)) => {
                let settings = AppSettings {
                    kv_path: self.kv_path.clone(),
                    layout: template.data.clone(),
                };

                let mut import_object =
                    self.import_object_create(&tx.app, &state, gas_left, host_ctx, &settings);

                self.import_object_extend(&mut import_object);

                let result =
                    self.do_exec_app(&tx, &template, &template_addr, &import_object, gas_left);

                let receipt = self.make_receipt(result);

                info!("receipt: {:?}", receipt);

                receipt
            }
        }
    }

    fn do_exec_app(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &TemplateAddr,
        import_object: &ImportObject,
        gas_left: MaybeGas,
    ) -> Result<(Option<State>, Vec<WasmValue>, MaybeGas), ExecAppError> {
        let module = self.compile_template(tx, &template, &template_addr, gas_left)?;

        let mut instance = self.instantiate(tx, template_addr, &module, import_object)?;

        self.init_instance_buffer(&tx.func_buf, &mut instance);

        let args = self.prepare_args_and_memory(tx);

        let func = match self.get_exported_func(tx, template_addr, &instance) {
            Err(e) => return Err(e),
            Ok(func) => func,
        };

        let result = func.call(&args);

        let gas_used = self.wasmer_instance_gas_used(&instance);

        if gas_used.is_err() {
            return Err(ExecAppError::OOG);
        }

        match result {
            Err(e) => Err(ExecAppError::ExecFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_idx: tx.func_idx,
                func_args: self.vec_to_str(&tx.func_args),
                reason: e.to_string(),
            }),
            Ok(returns) => {
                let storage = self.instance_storage_mut(&mut instance);
                let new_state = Some(storage.commit());

                let returns = self.cast_wasmer_func_returns(returns)?;
                let gas_used = gas_used.unwrap();

                Ok((new_state, returns, gas_used))
            }
        }
    }

    fn make_receipt(
        &self,
        result: Result<(Option<State>, Vec<WasmValue>, MaybeGas), ExecAppError>,
    ) -> ExecReceipt {
        match result {
            Err(e) => e.into(),
            Ok((new_state, returns, gas_used)) => ExecReceipt {
                success: true,
                error: None,
                returns: Some(returns),
                new_state,
                gas_used,
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

    #[inline]
    fn wasmer_instance_gas_used(
        &self,
        instance: &wasmer_runtime::Instance,
    ) -> Result<MaybeGas, OOGError> {
        helpers::wasmer_gas_used(instance)
    }

    fn cast_wasmer_func_returns(
        &self,
        returns: Vec<WasmerValue>,
    ) -> Result<Vec<WasmValue>, ExecAppError> {
        let mut values = Vec::new();

        for ret in returns.iter() {
            match ret {
                WasmerValue::I32(v) => values.push(WasmValue::I32(*v as u32)),
                WasmerValue::I64(v) => values.push(WasmValue::I64(*v as u64)),
                _ => unreachable!(),
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

        let func_name = instance
            .exports()
            .filter(|(_name, export)| matches!(export, Export::Function { .. }))
            .find_map(|(name, _)| {
                if func_idx == instance.resolve_func(&name).unwrap() {
                    Some(name)
                } else {
                    None
                }
            });

        if func_name.is_none() {
            // TOOD: ...
            panic!()
        }

        instance.exports.get(&func_name.unwrap()).or_else(|_e| {
            error!("Exported function: `{}` not found", func_idx);

            Err(ExecAppError::FuncNotFound {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_idx: *&tx.func_idx,
            })
        })
    }

    fn derive_func_index(&self, instance: &wasmer_runtime::Instance, tx: &AppTransaction) -> usize {
        let rel_func_index = tx.func_idx as usize;
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
    fn instance_storage_mut(&self, instance: &mut wasmer_runtime::Instance) -> &mut AppStorage {
        let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();
        helpers::wasmer_data_app_storage(wasmer_ctx.data)
    }

    fn import_object_create(
        &self,
        addr: &AppAddr,
        state: &State,
        gas_limit: MaybeGas,
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
            gas_limit,
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
        gas_left: MaybeGas,
    ) -> Result<wasmer_runtime::Module, ExecAppError> {
        info!("runtime `compile_template` (template={:?})", template_addr);

        let gas_metering = gas_left.is_some();
        let gas_left = gas_left.unwrap_or(0);

        svm_compiler::compile_program(&template.code, gas_left, gas_metering).or_else(|e| {
            error!("module compilation failed (template={:?})", template_addr);

            Err(ExecAppError::CompilationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                reason: e.to_string(),
            })
        })
    }

    /// Parse

    fn parse_deploy_template(&self, bytes: &[u8]) -> Result<AppTemplate, ParseError> {
        self.env.parse_deploy_template(bytes)
    }

    fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, ParseError> {
        self.env.parse_spawn_app(bytes)
    }

    fn parse_exec_app(&self, bytes: &[u8]) -> Result<AppTransaction, ParseError> {
        self.env.parse_exec_app(bytes)
    }

    /// Gas
    fn compute_install_template_gas(&self, bytes: &[u8], _template: &AppTemplate) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    fn compute_install_app_gas(&self, bytes: &[u8], _spawn: &SpawnApp) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    /// Helpers
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

    fn ensure_not_svm_ns(imports: &[(String, String, Export)]) {
        if imports.iter().any(|(ns, _, _)| ns == "svm") {
            panic!("Imports namespace can't be `svm` since it's a reserved name.")
        }
    }
}

impl<ENV, GE> Drop for DefaultRuntime<ENV, GE> {
    fn drop(&mut self) {
        info!("dropping DefaultRuntime...");
    }
}
