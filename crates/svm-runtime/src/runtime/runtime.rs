use log::{debug, error, info};
use std::ffi::c_void;

use svm_common::{Address, State};
use svm_contract::env::{ContractEnv, ContractEnvTypes};

use crate::contract_settings::ContractSettings;
use crate::ctx::SvmCtx;
use crate::ctx_data_wrapper::SvmCtxDataWrapper;
use crate::helpers;
use crate::runtime::{ContractExecError, Receipt};
use crate::vmcalls;

use svm_contract::{
    error::{ContractBuildError, TransactionBuildError},
    traits::ContractStore,
    transaction::Transaction,
    wasm::Contract,
};
use svm_storage::{ContractPages, ContractStorage};

use wasmer_runtime_core::{
    func,
    import::{ImportObject, Namespace},
};

pub struct Runtime<ENV> {
    pub env: ENV,
    pub storage_builder: Box<dyn Fn(&Address, &State, &ContractSettings) -> ContractStorage>,
}

impl<TY, ENV> Runtime<ENV>
where
    TY: ContractEnvTypes,
    ENV: ContractEnv<Types = TY>,
{
    pub fn new(
        env: ENV,
        storage_builder: Box<dyn Fn(&Address, &State, &ContractSettings) -> ContractStorage>,
    ) -> Self {
        Self {
            env,
            storage_builder,
        }
    }

    pub fn contract_build(&self, bytes: &[u8]) -> Result<Contract, ContractBuildError> {
        info!("runtime `contract_build`");

        <ENV as ContractEnv>::build_contract(bytes)
    }

    #[inline(always)]
    pub fn contract_deploy_validate(&self, contract: &Contract) -> Result<(), ContractBuildError> {
        // TODO:
        // validate the `wasm`. should use the `deterministic` feature of `wasmparser`.
        // (avoiding floats etc.)

        Ok(())
    }

    #[inline(always)]
    pub fn contract_compute_address(&self, contract: &Contract) -> Address {
        info!("runtime `contract_compute_address`");

        <ENV as ContractEnv>::compute_address(contract)
    }

    #[inline(always)]
    pub fn contract_store(&mut self, contract: &Contract, addr: &Address) {
        info!("runtime `contract_store`");

        self.env.store_contract(contract, addr);
    }

    #[inline(always)]
    pub fn transaction_build(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
        info!("runtime `transaction_build`");

        <ENV as ContractEnv>::build_transaction(bytes)
    }

    pub fn contract_exec(&self, tx: Transaction, import_object: &ImportObject) -> Receipt {
        info!("runtime `contract_exec`");

        let receipt = match self.do_contract_exec(&tx, import_object) {
            Err(e) => Receipt {
                success: false,
                error: Some(e),
                tx,
                results: Vec::new(),
                new_state: None,
            },
            Ok((state, results)) => Receipt {
                success: true,
                error: None,
                tx,
                results,
                new_state: Some(state),
            },
        };

        info!("receipt: {:?}", receipt);

        receipt
    }

    #[inline(always)]
    fn do_contract_exec(
        &self,
        tx: &Transaction,
        import_object: &ImportObject,
    ) -> Result<(State, Vec<wasmer_runtime::Value>), ContractExecError> {
        let contract = self.contract_load(tx)?;
        let module = self.contract_compile(&contract, &tx.contract)?;
        let mut instance = self.instantiate(&contract, &tx.contract, &module, import_object)?;
        let args = self.prepare_args_and_memory(tx, &mut instance);
        let func = self.get_exported_func(&instance, &tx.func_name)?;

        match func.call(&args) {
            Err(e) => Err(ContractExecError::ExecFailed),
            Ok(results) => {
                let storage = self.get_instance_svm_storage_mut(&mut instance);
                let state = storage.commit();
                Ok((state, results))
            }
        }
    }

    fn instantiate(
        &self,
        contract: &Contract,
        addr: &Address,
        module: &wasmer_runtime::Module,
        import_object: &ImportObject,
    ) -> Result<wasmer_runtime::Instance, ContractExecError> {
        info!("runtime `instantiate` (wasmer module instantiate)");

        let instantiate = module.instantiate(import_object);

        match instantiate {
            Err(e) => Err(ContractExecError::InstantiationFailed(addr.clone())),
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
            Err(e) => {
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
        helpers::wasmer_data_storage(wasmer_ctx.data)
    }

    pub fn open_contract_storage(
        &self,
        addr: &Address,
        state: &State,
        settings: &ContractSettings,
    ) -> ContractStorage {
        let storage_builder = &self.storage_builder;
        storage_builder(addr, state, settings)
    }

    pub fn import_object_create(
        &self,
        addr: &Address,
        state: &State,
        node_data: *const c_void,
        settings: &ContractSettings,
    ) -> ImportObject {
        debug!(
            "runtime `import_object_create` address={:?}, state={:?}, settings={:?}",
            addr, state, settings
        );

        let storage = self.open_contract_storage(addr, state, settings);

        let ctx = SvmCtx::new(SvmCtxDataWrapper::new(node_data), storage);
        let ctx = Box::leak(Box::new(ctx));

        let state_creator = move || {
            let node_data: *mut c_void = ctx as *const SvmCtx as *mut SvmCtx as _;

            let dtor: fn(*mut c_void) = |ctx_data| {
                let ctx_ptr = ctx_data as *mut SvmCtx;

                // triggers memory releasing
                unsafe { Box::from_raw(ctx_ptr) };
            };

            (node_data, dtor)
        };

        let mut import_object = ImportObject::new_with_data(state_creator);
        let mut ns = Namespace::new();

        vmcalls::insert_vmcalls(&mut ns);

        import_object.register("svm", ns);
        import_object
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
            Err(e) => {
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
