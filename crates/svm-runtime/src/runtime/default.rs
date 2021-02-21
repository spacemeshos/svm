use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

use log::{error, info};

use crate::env::{self, traits};

use env::{ExtApp, ExtSpawnApp, ExtTemplate};
use traits::{Env, EnvTypes};

use crate::error::ValidateError;
use crate::gas::GasEstimator;
use crate::storage::StorageBuilderFn;
use crate::vmcalls;
use crate::{Config, Context, ExternImport, Runtime};

use svm_codec::{app, template, ParseError};
use svm_ffi::svm_env_t;
use svm_layout::Layout;
use svm_storage::app::AppStorage;

use svm_types::gas::{MaybeGas, OOGError};
use svm_types::receipt::{self, ExecReceipt, Log, SpawnAppReceipt, TemplateReceipt};
use svm_types::{AppAddr, AuthorAddr, SpawnApp, SpawnerAddr, State, TemplateAddr, Type};
use svm_types::{RuntimeError, Transaction};

use wasmer::{Exports, Extern, ImportObject, Instance, Module, Store, WasmPtr, WasmTypeList};

use super::{Call, CallAddr, CallKind, Function, Outcome};

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
        let template = self.env.parse_deploy_template(bytes)?;
        let code = &template.code;

        svm_gas::validate_code(code).map_err(|e| e.into())
    }

    fn validate_app(&self, bytes: &[u8]) -> Result<(), ValidateError> {
        self.env
            .parse_spawn_app(bytes)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn validate_tx(&self, bytes: &[u8]) -> Result<Transaction, ValidateError> {
        let tx = self.env.parse_exec_app(bytes);

        tx.map_err(|e| e.into())
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

        let install_gas = self.compute_install_template_gas(bytes, &template);

        if gas_limit >= install_gas {
            let gas_used = MaybeGas::with(install_gas);

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

        let install_gas = self.compute_install_app_gas(bytes, &spawn);
        let gas_left = gas_limit - install_gas;

        match gas_left {
            Ok(gas_left) => {
                let app = ExtApp::new(spawn.app(), spawner);
                let addr = self.env.derive_app_address(&spawn);

                self.env.store_app(&app, &addr);

                let gas_used = install_gas.into();

                self.call_ctor(&spawn, &addr, gas_used, gas_left)
            }
            Err(..) => SpawnAppReceipt::new_oog(Vec::new()),
        }
    }

    fn exec_app(&self, tx: &Transaction, state: &State, gas_limit: MaybeGas) -> ExecReceipt {
        let call = Call {
            func_name: tx.func_name(),
            calldata: tx.calldata(),
            addr: CallAddr::with_app(tx.app_addr()),
            state,
            gas_used: MaybeGas::with(0),
            gas_left: gas_limit,
            within_spawn: false,
        };

        self.exec(&call)
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

    fn call_ctor(
        &mut self,
        spawn: &ExtSpawnApp,
        app_addr: &AppAddr,
        gas_used: MaybeGas,
        gas_left: MaybeGas,
    ) -> SpawnAppReceipt {
        let addr = CallAddr::new(spawn.template_addr(), app_addr);

        let call = Call {
            func_name: spawn.ctor_name(),
            calldata: spawn.ctor_data(),
            state: &State::zeros(),
            addr,
            within_spawn: true,
            gas_used,
            gas_left,
        };

        let receipt = self.exec(&call);

        receipt::into_spawn_app_receipt(receipt, app_addr)
    }

    fn install_template(&mut self, template: &ExtTemplate, gas_used: MaybeGas) -> TemplateReceipt {
        let addr = self.env.derive_template_address(template);

        self.env.store_template(template, &addr);

        TemplateReceipt::new(addr, gas_used)
    }

    fn exec(&self, call: &Call) -> ExecReceipt {
        info!("runtime `exec`");

        match self.load_template(call.app_addr()) {
            Ok(template) => {
                let store = svm_compiler::new_store();

                let storage = (self.storage_builder)(
                    call.app_addr(),
                    call.state(),
                    template.layout(),
                    &self.config,
                );

                let mut ctx = Context::new(
                    call.gas_left(),
                    storage,
                    call.template_addr(),
                    call.app_addr(),
                );

                let (import_object, host_envs) = self.create_import_object(&store, &mut ctx);
                self.drop_envs(host_envs);

                let out = self.do_exec(&call, &store, &ctx, &template, &import_object);

                let new_state = self.commit_changes(&ctx);

                self.make_receipt(out, Some(new_state))
            }
            Err(e) => ExecReceipt::from_err(e, Vec::new()),
        }
    }

    fn drop_envs(&self, mut host_envs: Vec<*mut svm_env_t>) {
        for env in host_envs.drain(..) {
            let ty = Type::of::<svm_env_t>();
            let _ = svm_ffi::from_raw(ty, env);
        }
    }

    fn do_exec(
        &self,
        call: &Call,
        store: &Store,
        ctx: &Context,
        template: &ExtTemplate,
        import_object: &ImportObject,
    ) -> Outcome<Vec<u8>> {
        let module = self.compile_template(store, ctx, &template, call.gas_left());

        if module.is_err() {
            return Outcome::Failure {
                err: module.unwrap_err(),
                logs: Vec::new(),
            };
        }

        let within_spawn = call.within_spawn();
        let is_ctor = template.is_ctor(call.func_name());

        if within_spawn && !is_ctor {
            let msg = "expected function to be a constructor";
            let err = self.func_not_allowed(ctx, call.func_name(), msg);

            return err.into();
        }

        if !within_spawn && is_ctor {
            let msg = "expected function to be a non-constructor";
            let err = self.func_not_allowed(ctx, call.func_name(), msg);

            return err.into();
        }

        let instance = self.instantiate(ctx, &module.unwrap(), import_object);
        if instance.is_err() {
            let err = instance.unwrap_err();
            return err.into();
        }

        let mut instance = instance.unwrap();

        let func = self.get_func::<(), ()>(&instance, ctx, call.func_name());

        if func.is_err() {
            let err = self.func_not_found(ctx, call.func_name());

            return err.into();
        }

        let mut out = self.call_with_alloc(&instance, ctx, call.calldata(), &func.unwrap(), &[]);

        if let Outcome::Failure { err, logs } = out {
            return Outcome::Failure { err, logs };
        }

        // TODO: assert that `out.returns()` is empty
        // an `endpoint` should always be of signature `[] -> []`

        if let Ok(gas_used) = self.instance_gas_used(&instance) {
            Outcome::Success {
                returns: self.take_returndata(ctx),
                gas_used,
                logs: out.take_logs(),
            }
        } else {
            Outcome::Failure {
                err: RuntimeError::OOG,
                logs: out.take_logs(),
            }
        }
    }

    fn call_alloc(&self, instance: &Instance, ctx: &Context, size: usize) -> Outcome<WasmPtr<u8>> {
        let func_name = "svm_alloc";

        let func = self.get_func::<u32, u32>(&instance, ctx, func_name);

        if func.is_err() {
            let err = self.func_not_found(ctx, func_name);

            return Outcome::Failure {
                err,
                logs: Vec::new(),
            };
        }

        let func = func.unwrap();
        let params: [wasmer::Val; 1] = [(size as i32).into()];

        let out = self.call(instance, ctx, &func, &params);

        out.map(|rets| {
            // TODO: assert that `rets.len() == 1`

            let ret = &rets[0];
            let ptr = ret.i32().unwrap();

            WasmPtr::new(ptr as u32)
        })
    }

    fn call_with_alloc<Args, Rets>(
        &self,
        instance: &Instance,
        ctx: &Context,
        calldata: &[u8],
        func: &Function<Args, Rets>,
        params: &[wasmer::Val],
    ) -> Outcome
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let outcome = self.call_alloc(instance, ctx, calldata.len());

        if let Outcome::Failure { err, logs } = outcome {
            return Outcome::Failure { err, logs };
        }

        // we assert that `svm_alloc` didn't touch the `returndata`
        // TODO: return an error instead of `panic`
        self.assert_no_returndata(ctx);

        let ptr = outcome.returns().clone();
        self.set_calldata(ctx, calldata, ptr);

        self.call(instance, ctx, func, params)
    }

    fn call<Args, Rets>(
        &self,
        instance: &Instance,
        ctx: &Context,
        func: &Function<Args, Rets>,
        params: &[wasmer::Val],
    ) -> Outcome
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let wasmer_func = func.wasmer_func();
        let returns = wasmer_func.call(params);

        let logs = self.take_logs(ctx);

        if returns.is_err() {
            let err = self.func_failed(ctx, func.name(), returns.unwrap_err());

            return Outcome::Failure { err, logs };
        }

        let gas_used = self.instance_gas_used(&instance);

        if let Ok(gas_used) = gas_used {
            let returns = returns.unwrap();

            Outcome::Success {
                gas_used,
                logs,
                returns,
            }
        } else {
            Outcome::Failure {
                err: RuntimeError::OOG,
                logs,
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

    fn take_logs(&self, ctx: &Context) -> Vec<Log> {
        ctx.borrow_mut().take_logs()
    }

    fn make_receipt(&self, out: Outcome<Vec<u8>>, new_state: Option<State>) -> ExecReceipt {
        match out {
            Outcome::Failure { err, logs } => ExecReceipt::from_err(err, logs),
            Outcome::Success {
                returns,
                gas_used,
                logs,
            } => ExecReceipt {
                version: 0,
                success: true,
                error: None,
                returndata: Some(returns),
                new_state,
                gas_used,
                logs,
            },
        }
    }

    fn set_memory(&self, ctx: &Context, instance: &Instance) {
        // TODO: raise when no exported memory exists
        let memory = instance.exports.get_memory("memory").unwrap();
        ctx.borrow_mut().set_memory(memory.clone());
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
        ctx: &Context,
        module: &Module,
        import_object: &ImportObject,
    ) -> Result<Instance, RuntimeError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        let instance = Instance::new(module, import_object);

        instance.map_err(|err| self.instantiation_failed(ctx, err))
    }

    fn get_func<'i, Args, Rets>(
        &self,
        instance: &'i Instance,
        ctx: &Context,
        func_name: &str,
    ) -> Result<Function<'i, Args, Rets>, RuntimeError>
    where
        Args: WasmTypeList,
        Rets: WasmTypeList,
    {
        let func = instance
            .exports
            .get_function(func_name)
            .map_err(|_| self.func_not_found(ctx, func_name))?;

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

    fn load_template(&self, app_addr: &AppAddr) -> Result<ExtTemplate, RuntimeError> {
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
    ) -> Result<Module, RuntimeError> {
        info!(
            "runtime `compile_template` (template={:?})",
            ctx.template_addr()
        );

        let gas_metering = gas_left.is_some();
        let gas_left = gas_left.unwrap_or(0);

        let module = svm_compiler::compile(store, template.code(), gas_left, gas_metering);
        module.map_err(|err| self.compilation_failed(ctx, err))
    }

    /// Gas
    fn compute_install_template_gas(&self, bytes: &[u8], _template: &ExtTemplate) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    fn compute_install_app_gas(&self, bytes: &[u8], spawn: &ExtSpawnApp) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    /// Errors

    #[inline]
    fn func_not_found(&self, ctx: &Context, func_name: &str) -> RuntimeError {
        RuntimeError::FuncNotFound {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
        }
    }

    #[inline]
    fn instantiation_failed(&self, ctx: &Context, err: wasmer::InstantiationError) -> RuntimeError {
        RuntimeError::InstantiationFailed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            msg: err.to_string(),
        }
    }

    #[inline]
    fn func_not_allowed(&self, ctx: &Context, func_name: &str, msg: &str) -> RuntimeError {
        RuntimeError::FuncNotAllowed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
            msg: msg.to_string(),
        }
    }

    #[inline]
    fn func_invalid_sig(&self, ctx: &Context, func_name: &str) -> RuntimeError {
        RuntimeError::FuncInvalidSignature {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
        }
    }

    #[inline]
    fn func_failed(
        &self,
        ctx: &Context,
        func_name: &str,
        err: wasmer::RuntimeError,
    ) -> RuntimeError {
        RuntimeError::FuncFailed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            func: func_name.to_string(),
            msg: err.to_string(),
        }
    }

    #[inline]
    fn compilation_failed(&self, ctx: &Context, err: wasmer::CompileError) -> RuntimeError {
        error!("module module failed (template={:?})", ctx.template_addr());

        RuntimeError::CompilationFailed {
            app_addr: ctx.app_addr().clone(),
            template_addr: ctx.template_addr().clone(),
            msg: err.to_string(),
        }
    }
}
