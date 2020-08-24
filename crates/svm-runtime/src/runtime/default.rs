use std::{ffi::c_void, fmt, marker::PhantomData, path::Path};

use log::{debug, error, info};

use crate::env::traits::{Env, EnvTypes};
use crate::{
    ctx::SvmCtx,
    error::ValidateError,
    gas::GasEstimator,
    helpers::{self, DataWrapper},
    storage::StorageBuilderFn,
    vmcalls, Config, Runtime,
};

use svm_codec::error::ParseError;
use svm_gas::Gas;
use svm_layout::DataLayout;
use svm_storage::app::AppStorage;
use svm_types::{
    gas::{MaybeGas, OOGError},
    receipt::{
        make_spawn_app_receipt, ExecReceipt, Log, ReceiptError, SpawnAppReceipt, TemplateReceipt,
    },
    AppAddr, AppTemplate, AppTransaction, AuthorAddr, CreatorAddr, HostCtx, SpawnApp, State,
    TemplateAddr, WasmValue,
};

use wasmer_runtime::Value as WasmerValue;
use wasmer_runtime::WasmPtr;
use wasmer_runtime_core::{
    export::Export,
    import::{ImportObject, Namespace},
    memory::Memory,
    types::MemoryDescriptor,
    units::Pages,
};

/// Default `Runtime` implementation based on `wasmer`.
pub struct DefaultRuntime<ENV, GE> {
    /// The runtime environment. Used mainly for managing app persistence.
    pub env: ENV,

    /// A raw pointer to host (a.k.a the `Full-Node` in the realm of Blockchain).
    pub host: *mut c_void,

    /// The runtime configuration
    pub config: Config,

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
            Err(..) => {
                let log = Log {
                    msg: format!(
                        "not enough gas (installation_gas = {}) for installation",
                        install_gas
                    )
                    .into_bytes(),

                    code: 1,
                };

                SpawnAppReceipt::new_oog(vec![log])
            }
            Ok(gas_left) => {
                let addr = self.install_app(&spawn, creator, &host_ctx);
                let gas_used = install_gas.into();

                self.call_ctor(creator, spawn, &addr, host_ctx, gas_used, gas_left)
            }
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

        let config = Config::new(kv_path);

        Self {
            env,
            host,
            config,
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
        layout: &DataLayout,
    ) -> AppStorage {
        (self.storage_builder)(addr, state, layout, &self.config)
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
        let addr = self.env.store_template(template, author, &host_ctx);
        TemplateReceipt::new(addr, gas_used)
    }

    fn install_app(
        &mut self,
        spawn: &SpawnApp,
        creator: &CreatorAddr,
        host_ctx: &HostCtx,
    ) -> AppAddr {
        self.env.store_app(spawn, creator, host_ctx)
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
            calldata: spawn.calldata,
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
            Err(e) => {
                let empty_logs = Vec::new();
                ExecReceipt::from_err(e, empty_logs)
            }
            Ok((template, template_addr, _author, _creator)) => {
                let mut import_object =
                    self.import_object_create(&template, &tx.app, &state, gas_left, host_ctx);

                self.import_object_extend(&mut import_object);

                let (result, logs) =
                    self.do_exec_app(&tx, &template, &template_addr, &import_object, gas_left);

                let receipt = self.make_receipt(result, logs);

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
    ) -> (
        Result<(Option<State>, Vec<WasmValue>, MaybeGas), ReceiptError>,
        Vec<Log>,
    ) {
        let empty_logs = Vec::new();

        let module = self.compile_template(tx, &template, &template_addr, gas_left);
        if let Err(err) = module {
            return (Err(err), empty_logs);
        }

        let instance = self.instantiate(tx, template_addr, &module.unwrap(), import_object);
        if let Err(err) = instance {
            return (Err(err), empty_logs);
        }

        let mut instance = instance.unwrap();

        let wasm_ptr = self.alloc_calldata(tx, template_addr, &mut instance);
        if let Err(err) = wasm_ptr {
            return (Err(err), empty_logs);
        }

        self.set_calldata(&tx.calldata, wasm_ptr.unwrap(), &mut instance);

        let func = match self.get_exported_func(tx, template_addr, &instance) {
            Err(e) => return (Err(e), empty_logs),
            Ok(func) => func,
        };
        let func_res = func.call(&[]);

        let logs = self.instance_logs(&instance);

        let gas_used = self.instance_gas_used(&instance);
        if gas_used.is_err() {
            return (Err(ReceiptError::OOG), logs);
        }

        let result = match func_res {
            Err(e) => Err(ReceiptError::FuncFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_idx: tx.func_idx,
                msg: e.to_string(),
            }),
            Ok(returns) => {
                let storage = self.instance_storage_mut(&mut instance);
                let new_state = Some(storage.commit());

                // TODO: return the `returndata` back
                let returns = Ok(Vec::new());

                if let Err(err) = returns {
                    return (Err(err), logs);
                }

                let gas_used = gas_used.unwrap();

                Ok((new_state, returns.unwrap(), gas_used))
            }
        };

        (result, logs)
    }

    fn make_receipt(
        &self,
        result: Result<(Option<State>, Vec<WasmValue>, MaybeGas), ReceiptError>,
        logs: Vec<Log>,
    ) -> ExecReceipt {
        match result {
            Err(e) => ExecReceipt::from_err(e, logs),
            Ok((new_state, returns, gas_used)) => ExecReceipt {
                success: true,
                error: None,
                returns: Some(Vec::new()),
                new_state,
                gas_used,
                logs,
            },
        }
    }

    fn alloc_calldata(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        instance: &mut wasmer_runtime::Instance,
    ) -> Result<WasmPtr<u8>, ReceiptError> {
        let alloc = instance.exports.get("svm_alloc");

        if alloc.is_err() {
            let err = ReceiptError::FuncNotFound {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func_idx: 0, // TODO: this field will be discarded once we get rid of the `func_idx`
                             // we'll use `func_name` (String instead).
            };

            return Err(err);
        }

        let alloc: wasmer_runtime::Func<(i32), i32> = alloc.unwrap();

        let size = tx.calldata.len() as i32;
        let res = alloc.call(size);

        if res.is_err() {
            let err = ReceiptError::FuncFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: "Allocation failed for `svm_alloc`".to_string(),
                func_idx: 0, // TODO: this field will be discarded once we get rid of the `func_idx`
                             // we'll use `func_name` (String instead).
            };

            return Err(err);
        }

        let offset: i32 = res.unwrap();
        Ok(WasmPtr::new(offset as u32))
    }

    fn set_calldata(
        &self,
        calldata: &[u8],
        ptr: WasmPtr<u8>,
        instance: &mut wasmer_runtime::Instance,
    ) {
        let ctx = instance.context_mut();
        let memory = ctx.memory(0);
        let offset = ptr.offset();

        // Each wasm instance memory contains at least one `WASM Page`. (A `Page` size is 64KB)
        // The `len(calldata)` will be less than the `WASM Page` size.
        //
        // In any case, the `alloc_wasmer_memory` is in charge of allocating enough memory
        // for the program to run (so we don't need to have any bounds-checking here).

        // TODO: add to `validate_template` checking that `calldata` doesn't exceed ???
        // (we'll need to decide on a `calldata` limit).
        //
        // See [issue #140](https://github.com/spacemeshos/svm/issues/140)
        let offset = ptr.offset() as usize;
        let len = calldata.len();
        let view = &memory.view::<u8>()[offset..(offset + len)];

        for (cell, &byte) in view.iter().zip(calldata.iter()) {
            cell.set(byte);
        }

        let svm_ctx = self.instance_svm_ctx(instance);

        svm_ctx.set_calldata(offset, len);
    }

    #[inline]
    fn instance_gas_used(&self, instance: &wasmer_runtime::Instance) -> Result<MaybeGas, OOGError> {
        helpers::wasmer_gas_used(instance)
    }

    #[inline]
    fn instance_logs(&self, instance: &wasmer_runtime::Instance) -> Vec<Log> {
        let ctx = instance.context();
        helpers::wasmer_data_logs(ctx.data)
    }

    fn instantiate(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        module: &wasmer_runtime::Module,
        import_object: &ImportObject,
    ) -> Result<wasmer_runtime::Instance, ReceiptError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        module.instantiate(import_object).or_else(|e| {
            Err(ReceiptError::InstantiationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: e.to_string(),
            })
        })
    }

    fn get_exported_func<'a>(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        instance: &'a wasmer_runtime::Instance,
    ) -> Result<wasmer_runtime::DynFunc<'a>, ReceiptError> {
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

            Err(ReceiptError::FuncNotFound {
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

    #[inline]
    fn instance_storage_mut(&self, instance: &mut wasmer_runtime::Instance) -> &mut AppStorage {
        let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();
        helpers::wasmer_data_app_storage(wasmer_ctx.data)
    }

    #[inline]
    fn instance_svm_ctx<'a>(&self, instance: &'a mut wasmer_runtime::Instance) -> &'a mut SvmCtx {
        let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();
        helpers::wasmer_data_svm(wasmer_ctx.data)
    }

    fn import_object_create(
        &self,
        template: &AppTemplate,
        app_addr: &AppAddr,
        state: &State,
        gas_limit: MaybeGas,
        host_ctx: HostCtx,
    ) -> ImportObject {
        debug!(
            "runtime `import_object_create` address={:?}, state={:?}, config={:?}",
            app_addr, state, self.config
        );

        let layout = &template.data;
        let storage = self.open_app_storage(app_addr, state, layout);
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

        let mem = self.alloc_wasmer_memory();
        ns.insert("memory", mem);

        vmcalls::wasmer_register(&mut ns);

        import_object.register("svm", ns);
    }

    fn load_template(
        &self,
        tx: &AppTransaction,
    ) -> Result<(AppTemplate, TemplateAddr, AuthorAddr, CreatorAddr), ReceiptError> {
        info!("runtime `load_template`");

        self.env
            .load_template_by_app(&tx.app)
            .ok_or_else(|| ReceiptError::AppNotFound(tx.app.clone()))
    }

    fn compile_template(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &TemplateAddr,
        gas_left: MaybeGas,
    ) -> Result<wasmer_runtime::Module, ReceiptError> {
        info!("runtime `compile_template` (template={:?})", template_addr);

        let gas_metering = gas_left.is_some();
        let gas_left = gas_left.unwrap_or(0);

        svm_compiler::compile_program(&template.code, gas_left, gas_metering).or_else(|e| {
            error!("module module failed (template={:?})", template_addr);

            Err(ReceiptError::CompilationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: e.to_string(),
            })
        })
    }

    /// Instance Memory

    fn alloc_wasmer_memory(&self) -> Memory {
        let minimum = Pages(1);
        let maximum = None;
        let shared = false;
        let desc = MemoryDescriptor::new(minimum, maximum, shared).unwrap();

        let memory = Memory::new(desc);
        memory.unwrap()
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
