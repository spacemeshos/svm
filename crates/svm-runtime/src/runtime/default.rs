use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

use log::{error, info};

use crate::env::traits::{Env, EnvTypes};
use crate::error::ValidateError;
use crate::gas::GasEstimator;
use crate::storage::StorageBuilderFn;
use crate::vmcalls;
use crate::{Config, Context, ExternImport, Runtime};

use svm_codec::ParseError;
use svm_ffi::svm_env_t;
use svm_layout::Layout;
use svm_storage::app::AppStorage;

use svm_types::gas::{MaybeGas, OOGError};
use svm_types::receipt::{self, ExecReceipt, Log, ReceiptError, SpawnAppReceipt, TemplateReceipt};

use svm_types::{
    AppAddr, AppTransaction, AuthorAddr, CreatorAddr, SpawnApp, State, Template, TemplateAddr, Type,
};

use wasmer::{
    Exports, Extern, Function, ImportObject, Instance, Module, NativeFunc, Store, WasmPtr,
};

/// Default `Runtime` implementation based on `Wasmer`.
pub struct DefaultRuntime<ENV, GE> {
    /// The runtime environment. Used mainly for managing app persistence.
    env: ENV,

    /// The runtime configuration
    config: Config,

    /// External imports (living in the so-called `Host` or `Node`) to be consumed by the App.
    imports: *const Vec<ExternImport>,

    /// builds a `AppStorage` instance.
    storage_builder: Box<StorageBuilderFn>,

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
        let code = &template.code;

        svm_gas::validate_code(code).map_err(|e| e.into())
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

    fn deploy_template(
        &mut self,
        bytes: &[u8],
        author: &AuthorAddr,
        gas_limit: MaybeGas,
    ) -> TemplateReceipt {
        info!("runtime `deploy_template`");

        let template = self.parse_deploy_template(bytes).unwrap();
        let install_gas = self.compute_install_template_gas(bytes, &template);

        if gas_limit >= install_gas {
            let gas_used = MaybeGas::with(install_gas);
            let gas_left = gas_limit;

            self.install_template(&template, author, gas_used, gas_left)
        } else {
            TemplateReceipt::new_oog()
        }
    }

    fn spawn_app(
        &mut self,
        bytes: &[u8],
        creator: &CreatorAddr,
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
                let addr = self.install_app(&spawn, creator);
                let gas_used = install_gas.into();

                self.call_ctor(creator, spawn, &addr, gas_used, gas_left)
            }
        }
    }

    fn exec_app(&self, bytes: &[u8], state: &State, gas_limit: MaybeGas) -> ExecReceipt {
        let tx = self.parse_exec_app(bytes).unwrap();
        let gas_used = MaybeGas::with(0);

        self.exec(&tx, state, gas_used, gas_limit, false)
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
        env: ENV,
        kv_path: P,
        imports: &Vec<ExternImport>,
        storage_builder: Box<StorageBuilderFn>,
    ) -> Self {
        let config = Config::new(kv_path);
        let imports = imports as *const _;

        Self {
            env,
            config,
            imports,
            storage_builder,
            phantom: PhantomData::<GE>,
        }
    }

    /// Initialize a new `AppStorage` and returndata it.
    /// This method is of `pub` visibility since it's also helpful for tests that want to
    /// observe that app storage data.
    pub fn open_app_storage(
        &self,
        addr: &AppAddr,
        state: &State,
        layout: &Layout,
    ) -> AppStorage {
        (self.storage_builder)(addr, state, layout, &self.config)
    }

    fn call_ctor(
        &mut self,
        creator: &CreatorAddr,
        spawn: SpawnApp,
        app_addr: &AppAddr,
        gas_used: MaybeGas,
        gas_left: MaybeGas,
    ) -> SpawnAppReceipt {
        let ctor = self.build_ctor_call(creator, spawn, app_addr);

        let ctor_receipt = self.exec(&ctor, &State::zeros(), gas_used, gas_left, true);

        receipt::into_spawn_app_receipt(ctor_receipt, app_addr)
    }

    fn install_template(
        &mut self,
        template: &Template,
        author: &AuthorAddr,
        gas_used: MaybeGas,
        _gas_left: MaybeGas,
    ) -> TemplateReceipt {
        let addr = self.env.store_template(template, author);

        TemplateReceipt::new(addr, gas_used)
    }

    fn install_app(&mut self, spawn: &SpawnApp, creator: &CreatorAddr) -> AppAddr {
        self.env.store_app(spawn, creator)
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
            func_name: spawn.ctor_name,
            calldata: spawn.calldata,
        }
    }

    fn exec(
        &self,
        tx: &AppTransaction,
        state: &State,
        _gas_used: MaybeGas,
        gas_left: MaybeGas,
        with_spawn: bool,
    ) -> ExecReceipt {
        info!("runtime `exec_app`");

        match self.load_template(&tx) {
            Err(e) => {
                let empty_logs = Vec::new();
                ExecReceipt::from_err(e, empty_logs)
            }
            Ok((template, template_addr, _author, _creator)) => {
                let store = svm_compiler::new_store();
                let mut ctx = self.create_context(&template, &tx.app, &state, gas_left);
                let (import_object, funcs_envs) = self.create_import_object(&store, &mut ctx);

                let (result, logs) = self._exec(
                    &store,
                    &ctx,
                    &tx,
                    &template,
                    &template_addr,
                    &import_object,
                    gas_left,
                    with_spawn,
                );

                self.funcs_envs_destroy(funcs_envs);

                let receipt = self.make_receipt(result, logs);

                info!("receipt: {:?}", receipt);

                receipt
            }
        }
    }

    fn funcs_envs_destroy(&self, mut funcs_envs: Vec<*mut svm_env_t>) {
        for func_env in funcs_envs.drain(..) {
            let ty = Type::of::<svm_env_t>();
            let _ = svm_ffi::from_raw(ty, func_env);
        }
    }

    fn _exec(
        &self,
        store: &Store,
        ctx: &Context,
        tx: &AppTransaction,
        template: &Template,
        template_addr: &TemplateAddr,
        import_object: &ImportObject,
        gas_left: MaybeGas,
        within_spawn: bool,
    ) -> (
        Result<(Option<State>, Option<Vec<u8>>, MaybeGas), ReceiptError>,
        Vec<Log>,
    ) {
        let empty_logs = Vec::new();

        let module = self.compile_template(store, tx, &template, &template_addr, gas_left);
        if let Err(err) = module {
            return (Err(err), empty_logs);
        }

        let func_name = &tx.func_name;

        let is_ctor = template.ctors.iter().any(|ctor| ctor == func_name);

        if within_spawn && !is_ctor {
            let err = ReceiptError::FuncNotAllowed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func: func_name.clone(),
                msg: "expected function to be a constructor".to_string(),
            };
            return (Err(err), empty_logs);
        }

        if !within_spawn && is_ctor {
            let err = ReceiptError::FuncNotAllowed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func: func_name.clone(),
                msg: "expected function to be a non-constructor".to_string(),
            };
            return (Err(err), empty_logs);
        }

        let instance = self.instantiate(tx, template_addr, &module.unwrap(), import_object);
        if let Err(err) = instance {
            return (Err(err), empty_logs);
        }

        let mut instance = instance.unwrap();

        self.set_memory(ctx, &mut instance);

        let wasm_ptr = self.alloc_calldata(tx, template_addr, &mut instance);
        if let Err(err) = wasm_ptr {
            return (Err(err), empty_logs);
        }

        // we assert that `svm_alloc` didn't touch the `returndata`
        // TODO: return an error instead of `panic`
        self.assert_no_returndata(ctx);

        self.set_calldata(ctx, &tx.calldata, wasm_ptr.unwrap());

        let func = match self.get_func(tx, template_addr, &instance) {
            Err(e) => return (Err(e), empty_logs),
            Ok(func) => func,
        };

        let func_res = func.call(&[]);
        let logs = self.take_logs(ctx);

        let gas_used = self.instance_gas_used(&instance);
        if gas_used.is_err() {
            return (Err(ReceiptError::OOG), logs);
        }

        let result = match func_res {
            Err(e) => Err(ReceiptError::FuncFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                func: tx.func_name.clone(),
                msg: e.to_string(),
            }),
            Ok(_returns) => {
                // TODO: assert that `returns` is empty

                let returndata = self.take_returndata(ctx);
                let new_state = self.commit_chages(ctx);

                Ok((Some(new_state), Some(returndata), gas_used.unwrap()))
            }
        };

        (result, logs)
    }

    #[inline]
    fn commit_chages(&self, ctx: &Context) -> State {
        let storage = &mut ctx.borrow_mut().storage;
        storage.commit()
    }

    #[inline]
    fn assert_no_returndata(&self, ctx: &Context) {
        assert!(ctx.borrow().returndata.is_none())
    }

    fn take_returndata(&self, ctx: &Context) -> Vec<u8> {
        let data = ctx.borrow().returndata;

        match data {
            Some((offset, len)) => self.read_memory(ctx, offset, len),
            None => Vec::new(),
        }
    }

    fn read_memory(&self, ctx: &Context, offset: usize, len: usize) -> Vec<u8> {
        let borrow = ctx.borrow();
        let memory = borrow.get_memory();

        // TODO: guard again out-of-bounds
        let view = memory.view::<u8>();
        let cells = &view[offset..(offset + len)];

        cells.iter().map(|c| c.get()).collect()
    }

    fn take_logs(&self, ctx: &Context) -> Vec<Log> {
        ctx.borrow_mut().take_logs()
    }

    fn make_receipt(
        &self,
        result: Result<(Option<State>, Option<Vec<u8>>, MaybeGas), ReceiptError>,
        logs: Vec<Log>,
    ) -> ExecReceipt {
        match result {
            Err(e) => ExecReceipt::from_err(e, logs),
            Ok((new_state, returndata, gas_used)) => ExecReceipt {
                version: 0,
                success: true,
                error: None,
                returndata,
                new_state,
                gas_used,
                logs,
            },
        }
    }

    fn set_memory(&self, ctx: &Context, instance: &mut Instance) {
        // TODO: raise when no exported memory
        let memory = instance.exports.get_memory("memory").unwrap();
        ctx.borrow_mut().set_memory(memory.clone());
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

        let alloc: NativeFunc<u32, u32> = alloc.unwrap();

        let size = tx.calldata.len() as u32;
        let offset = alloc.call(size);

        if offset.is_err() {
            let err = ReceiptError::FuncFailed {
                app_addr: tx.app.clone(),
                template_addr: template_addr.clone(),
                msg: "Allocation has failed for `svm_alloc`".to_string(),
                func: "svm_alloc".to_string(),
            };

            return Err(err);
        }

        let offset = offset.unwrap();
        Ok(WasmPtr::new(offset))
    }

    fn set_calldata(&self, ctx: &Context, calldata: &[u8], ptr: WasmPtr<u8>) {
        let (offset, len) = {
            let borrow = ctx.borrow();
            let memory = borrow.get_memory();

            // Each WASM instance memory contains at least one `WASM Page`. (A `Page` size is 64KB)
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

            // TODO: guard again out-of-bounds
            let view = &memory.view::<u8>()[offset..(offset + len)];

            for (cell, &byte) in view.iter().zip(calldata.iter()) {
                cell.set(byte);
            }

            (offset, len)
        };

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
        instance
            .exports
            .get_function(&tx.func_name)
            .or_else(|_err| {
                Err(ReceiptError::FuncNotFound {
                    app_addr: tx.app.clone(),
                    template_addr: template_addr.clone(),
                    func: tx.func_name.clone(),
                })
            })
    }

    fn create_context(
        &self,
        template: &Template,
        app_addr: &AppAddr,
        state: &State,
        gas_limit: MaybeGas,
    ) -> Context {
        let layout = &template.data;
        let storage = self.open_app_storage(app_addr, state, layout);

        Context::new(gas_limit, storage)
    }

    fn create_import_object(
        &self,
        store: &Store,
        ctx: &mut Context,
    ) -> (ImportObject, Vec<*mut svm_env_t>) {
        let mut import_object = ImportObject::new();
        let mut funcs_envs = Vec::new();

        let mut exports = HashMap::new();

        let imports: &[ExternImport] = unsafe { &*self.imports as _ };

        for import in imports.iter() {
            let namespace = import.namespace();
            let ns_exports = exports.entry(namespace).or_insert(Exports::new());

            let (export, func_env) = import.wasmer_export(store, ctx);

            funcs_envs.push(func_env);
            let ext = Extern::from_vm_export(store, export);

            ns_exports.insert(import.name(), ext);
        }

        for (ns, exports) in exports {
            import_object.register(ns, exports);
        }

        let mut svm = Exports::new();
        vmcalls::wasmer_register(store, ctx, &mut svm);
        import_object.register("svm", svm);

        (import_object, funcs_envs)
    }

    fn load_template(
        &self,
        tx: &AppTransaction,
    ) -> Result<(Template, TemplateAddr, AuthorAddr, CreatorAddr), ReceiptError> {
        info!("runtime `load_template`");

        self.env
            .load_template_by_app(&tx.app)
            .ok_or_else(|| ReceiptError::AppNotFound(tx.app.clone()))
    }

    fn compile_template(
        &self,
        store: &Store,
        tx: &AppTransaction,
        template: &Template,
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

    fn parse_deploy_template(&self, bytes: &[u8]) -> Result<Template, ParseError> {
        self.env.parse_deploy_template(bytes)
    }

    fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, ParseError> {
        self.env.parse_spawn_app(bytes)
    }

    fn parse_exec_app(&self, bytes: &[u8]) -> Result<AppTransaction, ParseError> {
        self.env.parse_exec_app(bytes)
    }

    /// Gas
    fn compute_install_template_gas(&self, bytes: &[u8], _template: &Template) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    fn compute_install_app_gas(&self, bytes: &[u8], _spawn: &SpawnApp) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }
}
