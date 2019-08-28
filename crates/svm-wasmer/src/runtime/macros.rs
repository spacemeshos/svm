#[macro_export]
macro_rules! include_svm_runtime {
    ($PAGE_CACHE: ident, $ENV: ty) => {
        $crate::include_wasmer_svm_vmcalls!($PAGE_CACHE);

        pub mod runtime {
            #[inline(always)]
            pub fn contract_build(
                bytes: &[u8],
            ) -> Result<svm_contract::wasm::WasmContract, svm_contract::ContractDeployError> {
                <$ENV as svm_contract::env::ContractEnv>::build_contract(&bytes)
            }

            #[inline(always)]
            pub fn contract_validate(
                contract: &svm_contract::wasm::WasmContract,
            ) -> Result<(), svm_contract::ContractDeployError> {
                // TODO:
                // validate the `wasm`. should use the `deterministic` feature of `wasmparser`.
                // (avoiding floats etc.)

                Ok(())
            }

            #[inline(always)]
            pub fn contract_store(contract: &svm_contract::wasm::WasmContract, env: &mut $ENV) {
                use svm_contract::env::ContractEnv;
                env.store_contract(&contract)
            }

            pub fn contract_exec<F>(
                tx: svm_contract::Tx,
                state: svm_common::State,
                env: &mut $ENV,
                import_object_gen: F,
            ) -> Result<(), $crate::runtime::ContractExecError>
            where
                F: Fn(
                    svm_common::Address,
                    svm_common::State,
                )
                    -> Result<wasmer_runtime::ImportObject, $crate::runtime::ContractExecError>,
            {
                use svm_contract::wasm::WasmContract;

                let contract = contract_load(&tx, env)?;
                let module = contract_compile(&contract)?;
                let import_object = import_object_gen(tx.contract, state)?;
                let mut instance = module_instantiate(&contract, &module, &import_object)?;
                let args = prepare_args_and_memory(&contract, &mut instance);
                let func = get_exported_func(&instance, &tx.func_name)?;

                let res = func.call(&args);

                match res {
                    Err(_) => Err($crate::runtime::ContractExecError::ExecFailed),
                    Ok(_) => Ok(()),
                }
            }

            fn contract_load(
                tx: &svm_contract::Tx,
                env: &mut $ENV,
            ) -> Result<svm_contract::wasm::WasmContract, $crate::runtime::ContractExecError> {
                use svm_contract::env::ContractEnv;
                use svm_contract::traits::ContractStore;
                use $crate::runtime::ContractExecError;

                let store = env.get_store();

                match store.load(tx.contract) {
                    None => Err(ContractExecError::NotFound(tx.contract)),
                    Some(contract) => Ok(contract),
                }
            }

            fn contract_compile(
                contract: &svm_contract::wasm::WasmContract,
            ) -> Result<wasmer_runtime::Module, $crate::runtime::ContractExecError> {
                use $crate::runtime::ContractExecError;

                let compile = wasmer_runtime::compile(&contract.wasm);
                match compile {
                    Err(_) => {
                        let addr = contract.address.unwrap();
                        Err(ContractExecError::CompilationFailed(addr))
                    }
                    Ok(module) => Ok(module),
                }
            }

            fn module_instantiate(
                contract: &svm_contract::wasm::WasmContract,
                module: &wasmer_runtime::Module,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<wasmer_runtime::Instance, $crate::runtime::ContractExecError> {
                use $crate::runtime::ContractExecError;

                let instantiate = module.instantiate(&import_object);

                match instantiate {
                    Err(_) => {
                        let addr = contract.address.unwrap();
                        Err(ContractExecError::InstantiationFailed(addr))
                    }
                    Ok(instance) => Ok(instance),
                }
            }

            fn get_exported_func<'a>(
                instance: &'a wasmer_runtime::Instance,
                func_name: &str,
            ) -> Result<wasmer_runtime::DynFunc<'a>, $crate::runtime::ContractExecError> {
                use $crate::runtime::ContractExecError;

                let func = instance.dyn_func(func_name);

                match func {
                    Err(_) => Err(ContractExecError::FuncNotFound(func_name.to_string())),
                    Ok(func) => Ok(func),
                }
            }

            fn prepare_args_and_memory(
                contract: &svm_contract::wasm::WasmContract,
                instance: &mut wasmer_runtime::Instance,
            ) -> Vec<wasmer_runtime::Value> {
                vec![]
            }
        }
    };
}
