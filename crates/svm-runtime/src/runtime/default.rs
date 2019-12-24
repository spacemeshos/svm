use std::ffi::c_void;

use log::{debug, error, info};

use crate::{
    ctx::SvmCtx,
    helpers,
    helpers::PtrWrapper,
    runtime::{ContractExecError, Receipt},
    settings::ContractSettings,
    traits::{Runtime, StorageBuilderFn},
};

use svm_common::{Address, State};
use svm_contract::{
    env::{ContractEnv, ContractEnvTypes},
    error::{ContractBuildError, TransactionBuildError},
    traits::ContractStore,
    transaction::Transaction,
    wasm::Contract,
};
use svm_storage::ContractStorage;

use wasmer_runtime_core::{
    export::Export,
    import::{ImportObject, Namespace},
};

/// Default `Runtime` implementation based on `wasmer`.
pub struct DefaultRuntime<ENV> {
    /// The runtime environment. Used mainly for managing contracts persistence/retrieval.
    pub env: ENV,

    /// A raw pointer to host (a.k.a the `Full-Node` in the realm of Blockchain).
    pub host: *const c_void,

    /// External imports (living inside the host) to be consumed by the wasm contracts.
    pub imports: Vec<(String, String, Export)>,

    /// Determined by the contract `Address` and `State` (contract state) and contract storage settings,
    /// builds a `ContractStorage` instance.
    pub storage_builder: Box<StorageBuilderFn>,
}

impl<TY, ENV> Runtime for DefaultRuntime<ENV>
where
    TY: ContractEnvTypes,
    ENV: ContractEnv<Types = TY>,
{
    fn contract_build(&self, bytes: &[u8]) -> Result<Contract, ContractBuildError> {
        info!("runtime `contract_build`");

        // TODO:
        // validate the `wasm`. should use the `deterministic` feature of `wasmparser`.
        // (avoiding floats etc.)
        self.env.build_contract(bytes)
    }

    #[inline(always)]
    fn contract_derive_address(&self, contract: &Contract) -> Address {
        info!("runtime `contract_compute_address`");

        self.env.compute_address(contract)
    }

    #[inline(always)]
    fn contract_deploy(&mut self, contract: &Contract, addr: &Address) {
        info!("runtime `contract_store`");

        self.env.store_contract(contract, addr);
    }

    #[inline(always)]
    fn transaction_build(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
        info!("runtime `transaction_build`");

        self.env.build_transaction(bytes)
    }

    fn transaction_exec(
        &self,
        tx: &Transaction,
        state: &State,
        settings: &ContractSettings,
    ) -> Receipt {
        info!("runtime `contract_exec`");

        let mut import_object = self.import_object_create(&tx.contract, state, settings);
        self.import_object_extend(&mut import_object);

        let receipt = match self.do_contract_exec(tx, &import_object) {
            Err(e) => Receipt {
                success: false,
                error: Some(e),
                tx: tx.clone(),
                results: Vec::new(),
                new_state: None,
            },
            Ok((state, results)) => Receipt {
                success: true,
                error: None,
                tx: tx.clone(),
                results,
                new_state: Some(state),
            },
        };

        info!("receipt: {:?}", receipt);

        receipt
    }
}

impl<TY, ENV> DefaultRuntime<ENV>
where
    TY: ContractEnvTypes,
    ENV: ContractEnv<Types = TY>,
{
    /// Initializes a new `DefaultRuntime` instance.
    pub fn new(
        host: *const c_void,
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

    /// Initialize a new `ContractStorage` and returns it.
    /// This method is of `pub` visibility since it's also helpful for tests that want to
    /// observe that contract storage data.
    pub fn open_contract_storage(
        &self,
        addr: &Address,
        state: &State,
        settings: &ContractSettings,
    ) -> ContractStorage {
        let sb = &self.storage_builder;
        sb(addr, state, settings)
    }

    #[inline(always)]
    fn do_contract_exec(
        &self,
        tx: &Transaction,
        import_object: &ImportObject,
    ) -> Result<(State, Vec<wasmer_runtime::Value>), ContractExecError> {
        let contract = self.contract_load(tx)?;
        let module = self.contract_compile(&contract, &tx.contract)?;
        let mut instance = self.instantiate(&tx.contract, &module, import_object)?;
        let args = self.prepare_args_and_memory(tx, &mut instance);
        let func = self.get_exported_func(&instance, &tx.func_name)?;

        match func.call(&args) {
            Err(_e) => Err(ContractExecError::ExecFailed),
            Ok(results) => {
                let storage = self.get_instance_svm_storage_mut(&mut instance);
                let state = storage.commit();
                Ok((state, results))
            }
        }
    }

    fn instantiate(
        &self,
        addr: &Address,
        module: &wasmer_runtime::Module,
        import_object: &ImportObject,
    ) -> Result<wasmer_runtime::Instance, ContractExecError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        let instantiate = module.instantiate(import_object);

        match instantiate {
            Err(_e) => Err(ContractExecError::InstantiationFailed(addr.clone())),
            Ok(instance) => Ok(instance),
        }
    }

    fn get_exported_func<'a>(
        &self,
        instance: &'a wasmer_runtime::Instance,
        func_name: &str,
    ) -> Result<wasmer_runtime::DynFunc<'a>, ContractExecError> {
        let func = instance.dyn_func(func_name);

        match func {
            Err(_e) => {
                error!("exported function: `{}` not found", func_name);

                Err(ContractExecError::FuncNotFound(func_name.to_string()))
            }
            Ok(func) => {
                info!("found exported function `{}`", func_name);
                Ok(func)
            }
        }
    }

    fn prepare_args_and_memory(
        &self,
        tx: &Transaction,
        instance: &mut wasmer_runtime::Instance,
    ) -> Vec<wasmer_runtime::Value> {
        use svm_contract::wasm::{WasmArgValue, WasmIntType};
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
    ) -> &mut svm_storage::ContractStorage {
        let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();
        helpers::wasmer_data_contract_storage(wasmer_ctx.data)
    }

    fn import_object_create(
        &self,
        addr: &Address,
        state: &State,
        settings: &ContractSettings,
    ) -> ImportObject {
        debug!(
            "runtime `import_object_create` address={:?}, state={:?}, settings={:?}",
            addr, state, settings
        );

        let storage = self.open_contract_storage(addr, state, settings);
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

    fn contract_load(&self, tx: &Transaction) -> Result<Contract, ContractExecError> {
        info!("runtime `contract_load`");

        let store = self.env.get_store();

        match store.load(&tx.contract) {
            None => Err(ContractExecError::NotFound(tx.contract.clone())),
            Some(contract) => Ok(contract),
        }
    }

    fn contract_compile(
        &self,
        contract: &Contract,
        addr: &Address,
    ) -> Result<wasmer_runtime::Module, ContractExecError> {
        info!("runtime `contract_compile` (addr={:?})", addr);

        let compile = svm_compiler::compile_program(&contract.wasm);

        match compile {
            Err(_e) => {
                error!("wasmer module compilation failed (addr={:?})", addr);
                Err(ContractExecError::CompilationFailed(addr.clone()))
            }
            Ok(module) => {
                info!("wasmer module compile succeeded");
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
