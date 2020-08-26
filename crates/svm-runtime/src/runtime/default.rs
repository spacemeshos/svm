use std::ffi::c_void;
use std::fmt;
use std::marker::PhantomData;
use std::path::Path;

use log::{debug, error, info};

use crate::{
    env::traits::{Env, EnvTypes},
    error::ValidateError,
    gas::GasEstimator,
    helpers::{self, DataWrapper},
    storage::StorageBuilderFn,
    vmcalls, Config, Context, Runtime,
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

use wasmer::{
    Export, Exports, Function, ImportObject, Instance, Memory, MemoryType, Module, NativeFunc,
    Pages, Store, Value, WasmPtr,
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
    /// Initializes a new `DefaultRuntime`.
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
            func: spawn.ctor,
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
                let store = svm_compiler::new_store();
                let memory = self.alloc_memory(&store);

                let ctx = self.create_ctx(memory, &template, &tx.app, &state, gas_left, host_ctx);
                let import_object = self.create_import_object(&store, &ctx);

                let (result, logs) = self.do_exec_app(
                    &store,
                    &ctx,
                    &tx,
                    &template,
                    &template_addr,
                    &import_object,
                    gas_left,
                );

                let receipt = self.make_receipt(result, logs);

                info!("receipt: {:?}", receipt);

                receipt
            }
        }
    }

    fn do_exec_app(
        &self,
        store: &Store,
        ctx: &Context,
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

        let module = self.compile_template(store, tx, &template, &template_addr, gas_left);
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

        self.set_calldata(ctx, &tx.calldata, wasm_ptr.unwrap());

        let func = match self.get_func(tx, template_addr, &instance) {
            Err(e) => return (Err(e), empty_logs),
            Ok(func) => func,
        };
        let func_res = func.call(&[]);

        let logs = ctx.borrow_mut().take_logs();

        let gas_used = self.instance_gas_used(&instance);
        if gas_used.is_err() {
            return (Err(ReceiptError::OOG), logs);
        }

        let result = match func_res {
            Err(e) => Err(ReceiptError::FuncFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func: tx.func.clone(),
                msg: e.to_string(),
            }),
            Ok(returns) => {
                let storage = &mut ctx.borrow_mut().storage;
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
        instance: &mut Instance,
    ) -> Result<WasmPtr<u8>, ReceiptError> {
        let alloc = instance.exports.get_native_function("svm_alloc");

        if alloc.is_err() {
            let err = ReceiptError::FuncNotFound {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func: "svm_alloc".to_string(),
            };

            return Err(err);
        }

        let alloc: NativeFunc<i32, i32> = alloc.unwrap();

        let size = tx.calldata.len() as i32;
        let offset = alloc.call(size);

        if offset.is_err() {
            let err = ReceiptError::FuncFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: "Allocation failed for `svm_alloc`".to_string(),
                func: "svm_alloc".to_string(),
            };

            return Err(err);
        }

        let offset = offset.unwrap() as u32;
        Ok(WasmPtr::new(offset))
    }

    fn set_calldata(&self, ctx: &Context, calldata: &[u8], ptr: WasmPtr<u8>) {
        let memory = &ctx.borrow().memory;
        let offset = ptr.offset();

        // Each wasm instance memory contains at least one `WASM Page`. (A `Page` size is 64KB)
        // The `len(calldata)` will be less than the `WASM Page` size.
        //
        // In any case, the `alloc_memory` is in charge of allocating enough memory
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

        ctx.borrow_mut().set_calldata(offset, len);
    }

    #[inline]
    fn instance_gas_used(&self, _instance: &Instance) -> Result<MaybeGas, OOGError> {
        // TODO: read `gas_used` out of `instance`
        Ok(MaybeGas::new())
    }

    fn instantiate(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        module: &Module,
        import_object: &ImportObject,
    ) -> Result<Instance, ReceiptError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        Instance::new(module, import_object).or_else(|e| {
            Err(ReceiptError::InstantiationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: e.to_string(),
            })
        })
    }

    fn get_func<'instance>(
        &self,
        tx: &AppTransaction,
        template_addr: &TemplateAddr,
        instance: &'instance Instance,
    ) -> Result<&'instance Function, ReceiptError> {
        instance.exports.get_function(&tx.func).or_else(|err| {
            Err(ReceiptError::FuncNotFound {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func: tx.func.clone(),
            })
        })
    }

    fn create_ctx(
        &self,
        memory: Memory,
        template: &AppTemplate,
        app_addr: &AppAddr,
        state: &State,
        gas_limit: MaybeGas,
        host_ctx: HostCtx,
    ) -> Context {
        let layout = &template.data;
        let storage = self.open_app_storage(app_addr, state, layout);
        let host_ctx = svm_common::into_raw(host_ctx);

        Context::new(
            memory,
            DataWrapper::new(self.host),
            DataWrapper::new(host_ctx),
            gas_limit,
            storage,
        )
    }

    fn create_import_object(&self, store: &Store, ctx: &Context) -> ImportObject {
        let mut import_object = ImportObject::new();

        let memory = ctx.borrow().memory.clone();

        let mut ns = Exports::new();

        ns.insert("memory", memory);

        vmcalls::wasmer_register(store, ctx, &mut ns);
        import_object.register("svm", ns);

        import_object
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
        store: &Store,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &TemplateAddr,
        gas_left: MaybeGas,
    ) -> Result<Module, ReceiptError> {
        info!("runtime `compile_template` (template={:?})", template_addr);

        let gas_metering = gas_left.is_some();
        let gas_left = gas_left.unwrap_or(0);

        svm_compiler::compile(store, &template.code, gas_left, gas_metering).or_else(|e| {
            error!("module module failed (template={:?})", template_addr);

            Err(ReceiptError::CompilationFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: e.to_string(),
            })
        })
    }

    fn alloc_memory(&self, store: &Store) -> Memory {
        let min = Pages(1);
        let max = None;
        let shared = false;
        let ty = MemoryType::new(min, max, shared);

        // TODO: return error when memory creation has failed.
        Memory::new(store, ty).unwrap()
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
