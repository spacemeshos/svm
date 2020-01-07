use std::convert::TryFrom;
use std::ffi::c_void;

use log::{debug, error, info};

use crate::{
    ctx::SvmCtx,
    error::{DeployTemplateError, ExecAppError, SpawnAppError},
    helpers,
    helpers::PtrWrapper,
    runtime::Receipt,
    settings::AppSettings,
    traits::{Runtime, StorageBuilderFn},
    value::Value,
};

use svm_app::{
    traits::{Env, EnvTypes},
    types::{App, AppTemplate, AppTransaction},
};
use svm_common::{Address, State};
use svm_storage::AppStorage;

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
    fn deploy_template(&mut self, bytes: &[u8]) -> Result<Address, DeployTemplateError> {
        info!("runtime `deploy_template`");

        match self.env.parse_template(bytes) {
            Err(e) => Err(DeployTemplateError::ParseFailed(e)),
            Ok(template) => match self.env.store_template(&template) {
                Err(e) => Err(DeployTemplateError::StoreFailed(e)),
                Ok(addr) => Ok(addr),
            },
        }
    }

    fn spawn_app(&mut self, bytes: &[u8]) -> Result<Address, SpawnAppError> {
        info!("runtime `spawn_app`");

        match self.env.parse_app(bytes) {
            Err(e) => return Err(SpawnAppError::ParseFailed(e)),
            Ok(app) => match self.env.store_app(&app) {
                Err(e) => Err(SpawnAppError::StoreFailed(e)),
                Ok(addr) => Ok(addr),
            },
        }
    }

    fn parse_exec_app(&self, bytes: &[u8]) -> Result<AppTransaction, ExecAppError> {
        match self.env.parse_app_tx(bytes) {
            Err(e) => Err(ExecAppError::ParseFailed(e)),
            Ok(tx) => Ok(tx),
        }
    }

    fn exec_app(&self, tx: AppTransaction, state: State) -> Result<Receipt, ExecAppError> {
        info!("runtime `exec_app`");

        let (template, template_addr) = self.load_template(&tx)?;

        let settings = AppSettings {
            pages_count: template.pages_count,
        };

        let mut import_object = self.import_object_create(&tx.app, &state, &settings);
        self.import_object_extend(&mut import_object);

        let receipt = match self.do_exec_app(&tx, &template, &template_addr, &import_object) {
            Err(e) => Receipt {
                tx,
                success: false,
                error: Some(e),
                results: Vec::new(),
                new_state: None,
            },
            Ok((state, results)) => Receipt {
                tx,
                success: true,
                error: None,
                results,
                new_state: Some(state),
            },
        };

        info!("receipt: {:?}", receipt);

        Ok(receipt)
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

    fn do_exec_app(
        &self,
        tx: &AppTransaction,
        template: &AppTemplate,
        template_addr: &Address,
        import_object: &ImportObject,
    ) -> Result<(State, Vec<Value>), ExecAppError> {
        let module = self.compile_template(&template, &template_addr)?;
        let mut instance = self.instantiate(&tx, &module, import_object)?;
        let args = self.prepare_args_and_memory(tx, &mut instance);
        let func = self.get_exported_func(&instance, &tx.func_name)?;

        match func.call(&args) {
            Err(e) => Err(ExecAppError::ExecFailed),
            Ok(results) => {
                let storage = self.get_instance_svm_storage_mut(&mut instance);
                let state = storage.commit();
                let results = self.cast_wasmer_results(results)?;

                Ok((state, results))
            }
        }
    }

    fn cast_wasmer_results(
        &self,
        results: Vec<wasmer_runtime::Value>,
    ) -> Result<Vec<Value>, ExecAppError> {
        let mut values = Vec::new();

        for wasmer_val in results.iter() {
            match Value::try_from(wasmer_val) {
                Err(_e) => {
                    return Err(ExecAppError::InvalidResultValue(format!(
                        "{:?}",
                        wasmer_val
                    )))
                }
                Ok(v) => values.push(v),
            }
        }

        Ok(values)
    }

    fn instantiate(
        &self,
        tx: &AppTransaction,
        module: &wasmer_runtime::Module,
        import_object: &ImportObject,
    ) -> Result<wasmer_runtime::Instance, ExecAppError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        let instantiate = module.instantiate(import_object);

        match instantiate {
            Err(e) => Err(ExecAppError::InstantiationFailed(tx.app.clone())),
            Ok(instance) => Ok(instance),
        }
    }

    fn get_exported_func<'a>(
        &self,
        instance: &'a wasmer_runtime::Instance,
        func_name: &str,
    ) -> Result<wasmer_runtime::DynFunc<'a>, ExecAppError> {
        let func = instance.dyn_func(func_name);

        match func {
            Err(_e) => {
                error!("Exported function: `{}` not found", func_name);

                Err(ExecAppError::FuncNotFound(func_name.to_string()))
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
        use svm_app::types::{WasmArgValue, WasmIntType};
        use wasmer_runtime::Value;

        debug!("runtime `prepare_args_and_memory`");

        let memory = instance.context_mut().memory(0);
        let mut mem_offset = 0;

        let mut wasmer_args = Vec::with_capacity(tx.func_args.len());

        for arg in tx.func_args.iter() {
            let wasmer_arg = match arg {
                WasmArgValue::I32(v) => Value::I32(*v as i32),
                WasmArgValue::I64(v) => Value::I64(*v as i64),
                WasmArgValue::Fixed(ty, buf) => {
                    let buf_mem_start = mem_offset;

                    let view = memory.view();

                    for byte in buf.into_iter() {
                        view[mem_offset].set(*byte);
                        mem_offset += 1;
                    }

                    match ty {
                        WasmIntType::I32 => Value::I32(buf_mem_start as i32),
                        WasmIntType::I64 => Value::I64(buf_mem_start as i64),
                    }
                }
                WasmArgValue::Slice(..) => unimplemented!(),
            };

            wasmer_args.push(wasmer_arg);
        }

        debug!("wasmer args={:?}", wasmer_args);

        wasmer_args
    }

    #[inline(always)]
    fn get_instance_svm_storage_mut(
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
        settings: &AppSettings,
    ) -> ImportObject {
        debug!(
            "runtime `import_object_create` address={:?}, state={:?}, settings={:?}",
            addr, state, settings
        );

        let storage = self.open_app_storage(addr, state, settings);
        let svm_ctx = SvmCtx::new(PtrWrapper::new(self.host), storage);
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
        // TODO: validate that `self.imports` don't use `svm` as for imports namespaces.

        import_object.extend(self.imports.clone());

        let mut ns = Namespace::new();
        crate::vmcalls::insert_vmcalls(&mut ns);

        import_object.register("svm", ns);
    }

    fn load_template(&self, tx: &AppTransaction) -> Result<(AppTemplate, Address), ExecAppError> {
        info!("runtime `load_template`");

        match self.env.load_template_by_app(&tx.app) {
            None => Err(ExecAppError::AppNotFound(tx.app.clone())),
            Some(res) => Ok(res),
        }
    }

    fn compile_template(
        &self,
        template: &AppTemplate,
        addr: &Address,
    ) -> Result<wasmer_runtime::Module, ExecAppError> {
        info!("runtime `compile_template` (addr={:?})", addr);

        match svm_compiler::compile_program(&template.code) {
            Err(e) => {
                error!("wasmer module compilation failed (addr={:?})", addr);
                Err(ExecAppError::CompilationFailed(addr.clone()))
            }
            Ok(module) => {
                debug!("wasmer module compile succeeded (addr={:?}", addr);
                Ok(module)
            }
        }
    }
}

impl<ENV> Drop for DefaultRuntime<ENV> {
    fn drop(&mut self) {
        info!("dropping Runtime...");
    }
}
