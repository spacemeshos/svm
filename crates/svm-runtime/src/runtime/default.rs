use std::marker::PhantomData;
use std::path::Path;
use std::{collections::HashMap, todo};

use log::{error, info};

use crate::env::{self, traits};

use env::{ExtApp, ExtSpawnApp, ExtTemplate};
use traits::{Env, EnvTypes};

use crate::error::ValidateError;
use crate::gas::GasEstimator;
use crate::storage::StorageBuilderFn;
use crate::vmcalls;
use crate::{Config, Context, ExternImport, Runtime};

use svm_ffi::svm_env_t;
use svm_layout::Layout;
use svm_storage::app::AppStorage;

use svm_types::gas::{MaybeGas, OOGError};
use svm_types::receipt::{self, ExecReceipt, Log, SpawnAppReceipt, TemplateReceipt};
use svm_types::{AppAddr, AuthorAddr, SpawnerAddr, State, Type};
use svm_types::{RuntimeError, Transaction};

use wasmer::{Exports, Extern, ImportObject, Instance, Module, Store, WasmPtr, WasmTypeList};

use super::{Call, Failure, Function, Outcome};

type Result<T> = std::result::Result<Outcome<T>, Failure>;

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
    fn validate_template(&self, bytes: &[u8]) -> std::result::Result<(), ValidateError> {
        let template = self.env.parse_deploy_template(bytes)?;
        let code = &template.code;

        svm_gas::validate_code(code).map_err(|e| e.into())
    }

    fn validate_app(&self, bytes: &[u8]) -> std::result::Result<(), ValidateError> {
        self.env
            .parse_spawn_app(bytes)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn validate_tx(&self, bytes: &[u8]) -> std::result::Result<Transaction, ValidateError> {
        self.env.parse_exec_app(bytes).map_err(|e| e.into())
    }

    fn deploy_template(
        &mut self,
        bytes: &[u8],
        author: &AuthorAddr,
        gas_limit: MaybeGas,
    ) -> TemplateReceipt {
        info!("runtime `deploy_template`");

        let base = self.env.parse_deploy_template(bytes).unwrap();
        let template = ExtTemplate::new(base, author);

        let install_price = self.template_installation_price(bytes, &template);

        if gas_limit >= install_price {
            let gas_used = MaybeGas::with(install_price);

            self.install_template(&template, gas_used)
        } else {
            TemplateReceipt::new_oog()
        }
    }

    fn spawn_app(
        &mut self,
        bytes: &[u8],
        spawner: &SpawnerAddr,
        gas_limit: MaybeGas,
    ) -> SpawnAppReceipt {
        info!("runtime `spawn_app`");

        let base = self.env.parse_spawn_app(bytes).unwrap();
        let spawn = ExtSpawnApp::new(base, spawner);

        let payload_price = self.spawn_payload_price(bytes, &spawn);
        let gas_left = gas_limit - payload_price;

        match gas_left {
            Ok(gas_left) => {
                let app = ExtApp::new(spawn.app(), spawner);
                let addr = self.env.derive_app_address(&spawn);

                self.env.store_app(&app, &addr);

                let gas_used = payload_price.into();

                self.call_ctor(&spawn, &addr, gas_used, gas_left)
            }
            Err(..) => SpawnAppReceipt::new_oog(Vec::new()),
        }
    }

    fn exec_verify(&self, tx: &Transaction, state: &State, gas_limit: MaybeGas) -> ExecReceipt {
        todo!()
    }

    fn exec_tx(&self, tx: &Transaction, state: &State, gas_limit: MaybeGas) -> ExecReceipt {
        let app_addr = tx.app_addr();
        let template_addr = self.env.find_template_addr(app_addr);

        if let Some(template_addr) = template_addr {
            let call = Call {
                func_name: tx.func_name(),
                calldata: tx.calldata(),
                template_addr: &template_addr,
                app_addr,
                state,
                gas_used: MaybeGas::with(0),
                gas_left: gas_limit,
                within_spawn: false,
            };

            self.exec_call::<(), ()>(&call)
        } else {
            unreachable!("Should have failed earlier when doing `validate_tx`");
        }
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

    fn outcome_to_receipt(
        &self,
        ctx: &Context,
        mut out: Outcome<Box<[wasmer::Val]>>,
    ) -> ExecReceipt {
        let returndata = self.take_returndata(ctx);
        let new_state = self.commit_changes(&ctx);

        ExecReceipt {
            version: 0,
            success: true,
            error: None,
            returndata: Some(returndata),
            new_state: Some(new_state),
            gas_used: out.gas_used(),
            logs: out.take_logs(),
        }
    }

    fn failure_to_receipt(&self, mut fail: Failure) -> ExecReceipt {
        let logs = fail.take_logs();
        let err = fail.take_error();

        ExecReceipt::from_err(err, logs)
    }

    /// Opens the `AppStorage` associated with the input params.
    pub fn open_storage(&self, app_addr: &AppAddr, state: &State, layout: &Layout) -> AppStorage {
        (self.storage_builder)(app_addr, state, layout, &self.config)
    }

    fn call_ctor(
        &mut self,
        spawn: &ExtSpawnApp,
        app_addr: &AppAddr,
        gas_used: MaybeGas,
        gas_left: MaybeGas,
    ) -> SpawnAppReceipt {
        let template_addr = spawn.template_addr();

        let call = Call {
            func_name: spawn.ctor_name(),
            calldata: spawn.ctor_data(),
            state: &State::zeros(),
            template_addr: &template_addr,
            app_addr,
            within_spawn: true,
            gas_used,
            gas_left,
        };

        let receipt = self.exec_call::<(), ()>(&call);

        receipt::into_spawn_app_receipt(receipt, app_addr)
    }

    fn install_template(&mut self, template: &ExtTemplate, gas_used: MaybeGas) -> TemplateReceipt {
        let addr = self.env.derive_template_address(template);

        self.env.store_template(template, &addr);

        TemplateReceipt::new(addr, gas_used)
    }

    fn exec_call<Args, Rets>(&self, call: &Call) -> ExecReceipt {
        let result =
            self.exec::<(), (), _, _>(&call, |ctx, mut out| self.outcome_to_receipt(ctx, out));

        result.unwrap_or_else(|fail| self.failure_to_receipt(fail))
    }

    fn exec<Args, Rets, F, T>(&self, call: &Call, f: F) -> std::result::Result<T, Failure>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
        F: Fn(&Context, Outcome<Box<[wasmer::Val]>>) -> T,
    {
        info!("runtime `exec`");

        match self.load_template(call.app_addr()) {
            Ok(template) => {
                let storage = self.open_storage(call.app_addr(), call.state(), template.layout());

                let mut ctx = Context::new(
                    call.gas_left(),
                    storage,
                    call.template_addr(),
                    call.app_addr(),
                );

                let store = svm_compiler::new_store();

                let (import_object, host_envs) = self.create_import_object(&store, &mut ctx);

                let result =
                    self.exec_::<Args, Rets>(&call, &store, &ctx, &template, &import_object);

                self.drop_envs(host_envs);

                match result {
                    Ok(out) => Ok(f(&ctx, out)),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    fn drop_envs(&self, mut host_envs: Vec<*mut svm_env_t>) {
        for env in host_envs.drain(..) {
            let ty = Type::of::<svm_env_t>();
            let _ = svm_ffi::from_raw(ty, env);
        }
    }

    fn exec_<Args, Rets>(
        &self,
        call: &Call,
        store: &Store,
        ctx: &Context,
        template: &ExtTemplate,
        import_object: &ImportObject,
    ) -> Result<Box<[wasmer::Val]>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        self.validate_call(call, template, ctx)?;

        let module = self.compile_template(store, ctx, &template, call.gas_left())?;
        let instance = self.instantiate(ctx, &module, import_object)?;

        self.set_memory(ctx, &instance);

        let func = self.get_func::<Args, Rets>(&instance, ctx, call.func_name())?;

        let mut out = self.call_with_alloc(&instance, ctx, call.calldata(), &func, &[])?;

        let logs = out.take_logs();

        match self.instance_gas_used(&instance) {
            Ok(gas_used) => {
                let returns = out.take_returns();
                let out = Outcome::new(returns, gas_used, logs);

                Ok(out)
            }
            Err(..) => {
                let err = Failure::new(RuntimeError::OOG, out.take_logs());

                Err(err)
            }
        }
    }

    fn call_with_alloc<Args, Rets>(
        &self,
        instance: &Instance,
        ctx: &Context,
        calldata: &[u8],
        func: &Function<Args, Rets>,
        params: &[wasmer::Val],
    ) -> Result<Box<[wasmer::Val]>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let out = self.call_alloc(instance, ctx, calldata.len())?;

        // we assert that `svm_alloc` didn't touch the `returndata`
        // TODO: return an error instead of `panic`
        self.assert_no_returndata(ctx);

        let wasm_ptr = out.returns();
        self.set_calldata(ctx, calldata, wasm_ptr);

        self.call(instance, ctx, func, params)
    }

    fn call_alloc(&self, instance: &Instance, ctx: &Context, size: usize) -> Result<WasmPtr<u8>> {
        let func_name = "svm_alloc";

        let func = self.get_func::<u32, u32>(&instance, ctx, func_name);

        if func.is_err() {
            let err = self.func_not_found(ctx, func_name);

            return Err(err);
        }

        let func = func.unwrap();
        let params: [wasmer::Val; 1] = [(size as i32).into()];

        let out = self.call(instance, ctx, &func, &params)?;

        let out = out.map(|rets| {
            let ret = &rets[0];
            let offset = ret.i32().unwrap() as u32;

            WasmPtr::new(offset)
        });

        Ok(out)
    }

    fn call<Args, Rets>(
        &self,
        instance: &Instance,
        ctx: &Context,
        func: &Function<Args, Rets>,
        params: &[wasmer::Val],
    ) -> Result<Box<[wasmer::Val]>>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let wasmer_func = func.wasmer_func();
        let returns = wasmer_func.call(params);
        let logs = ctx.borrow_mut().take_logs();

        if returns.is_err() {
            let err = self.func_failed(ctx, func.name(), returns.unwrap_err(), logs);

            return Err(err);
        }

        match self.instance_gas_used(&instance) {
            Ok(gas_used) => {
                let out = Outcome::new(returns.unwrap(), gas_used, logs);

                Ok(out)
            }
            Err(..) => {
                let err = Failure::new(RuntimeError::OOG, logs);

                Err(err)
            }
        }
    }

    #[inline]
    fn commit_changes(&self, ctx: &Context) -> State {
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

    fn set_memory(&self, ctx: &Context, instance: &Instance) {
        // TODO: raise when no exported memory exists
        let memory = instance.exports.get_memory("memory").unwrap();
        ctx.borrow_mut().set_memory(memory.clone());
    }

    fn set_calldata(&self, ctx: &Context, calldata: &[u8], wasm_ptr: WasmPtr<u8>) {
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
            let offset = wasm_ptr.offset() as usize;
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
    fn instance_gas_used(&self, _instance: &Instance) -> std::result::Result<MaybeGas, OOGError> {
        // TODO: read `gas_used` out of `instance`
        Ok(MaybeGas::new())
    }

    fn instantiate(
        &self,
        ctx: &Context,
        module: &Module,
        import_object: &ImportObject,
    ) -> std::result::Result<Instance, Failure> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        let instance = Instance::new(module, import_object);

        instance.map_err(|err| self.instantiation_failed(ctx, err))
    }

    fn get_func<'i, Args, Rets>(
        &self,
        instance: &'i Instance,
        ctx: &Context,
        func_name: &'i str,
    ) -> std::result::Result<Function<'i, Args, Rets>, Failure>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let func = instance.exports.get_function(func_name);

        if func.is_err() {
            let err = self.func_not_found(ctx, func_name);

            return Err(err);
        }

        let func = func.unwrap();
        let native = func.native::<Args, Rets>();

        if native.is_err() {
            let err = self.func_invalid_sig(ctx, func_name);

            return Err(err);
        }

        let func = Function::new(func, func_name);

        Ok(func)
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

    fn load_template(&self, app_addr: &AppAddr) -> std::result::Result<ExtTemplate, RuntimeError> {
        info!("runtime `load_template`");

        let template = self.env.load_template_by_app(app_addr);

        template.ok_or_else(|| RuntimeError::AppNotFound(app_addr.clone()))
    }

    fn compile_template(
        &self,
        store: &Store,
        ctx: &Context,
        template: &ExtTemplate,
        gas_left: MaybeGas,
    ) -> std::result::Result<Module, Failure> {
        info!(
            "runtime `compile_template` (template={:?})",
            ctx.template_addr()
        );

        let gas_metering = gas_left.is_some();
        let gas_left = gas_left.unwrap_or(0);

        let module = svm_compiler::compile(store, template.code(), gas_left, gas_metering);

        module.map_err(|err| self.compilation_failed(ctx, err))
    }

    fn validate_call(
        &self,
        call: &Call,
        template: &ExtTemplate,
        ctx: &Context,
    ) -> std::result::Result<(), Failure> {
        let spawning = call.within_spawn();
        let ctor = template.is_ctor(call.func_name());

        if spawning && !ctor {
            let msg = "expected function to be a constructor";
            let err = self.func_not_allowed(ctx, call.func_name(), msg);

            return Err(err);
        }

        if !spawning && ctor {
            let msg = "expected function to be a non-constructor";
            let err = self.func_not_allowed(ctx, call.func_name(), msg);

            return Err(err);
        }

        Ok(())
    }

    /// Gas
    fn template_installation_price(&self, bytes: &[u8], _template: &ExtTemplate) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    fn spawn_payload_price(&self, bytes: &[u8], _spawn: &ExtSpawnApp) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    /// Errors

    #[inline]
    fn func_not_found(&self, ctx: &Context, func_name: &str) -> Failure {
        RuntimeError::FuncNotFound {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
        }
        .into()
    }

    #[inline]
    fn instantiation_failed(&self, ctx: &Context, err: wasmer::InstantiationError) -> Failure {
        RuntimeError::InstantiationFailed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            msg: err.to_string(),
        }
        .into()
    }

    #[inline]
    fn func_not_allowed(&self, ctx: &Context, func_name: &str, msg: &str) -> Failure {
        RuntimeError::FuncNotAllowed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
            msg: msg.to_string(),
        }
        .into()
    }

    #[inline]
    fn func_invalid_sig(&self, ctx: &Context, func_name: &str) -> Failure {
        RuntimeError::FuncInvalidSignature {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
        }
        .into()
    }

    #[inline]
    fn func_failed(
        &self,
        ctx: &Context,
        func_name: &str,
        err: wasmer::RuntimeError,
        logs: Vec<Log>,
    ) -> Failure {
        let err = RuntimeError::FuncFailed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
            msg: err.to_string(),
        };

        Failure::new(err, logs)
    }

    #[inline]
    fn compilation_failed(&self, ctx: &Context, err: wasmer::CompileError) -> Failure {
        error!("module module failed (template={:?})", ctx.template_addr());

        RuntimeError::CompilationFailed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            msg: err.to_string(),
        }
        .into()
    }
}
