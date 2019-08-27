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
                // validates the `wasm`. should use the `deterministic` feature of `wasmparser`.
                // (avoiding floats etc.)
                unimplemented!()
            }

            #[inline(always)]
            pub fn contract_store(contract: &svm_contract::wasm::WasmContract, env: &mut $ENV) {
                use svm_contract::env::ContractEnv;
                env.store_contract(&contract)
            }

            pub fn contract_exec<F>(tx: svm_contract::Tx, env: &mut $ENV, import_object_gen: F)
            where
                F: Fn(svm_common::Address, svm_common::State) -> wasmer_runtime::ImportObject,
            {
                use svm_common::{Address, State};
                use svm_contract::env::ContractEnv;
                use svm_contract::traits::ContractStore;

                let store = env.get_store();

                match store.load(tx.contract) {
                    None => {
                        // should return a failure
                        // and the `tx.sender` should pay the maximum tx gas ??
                    }
                    Some(contract) => {
                        let compile = wasmer_runtime::compile(&contract.wasm);

                        match compile {
                            Err(_) => {
                                // wasm is invalid
                            }
                            Ok(module) => {
                                let import_object = import_object_gen(tx.contract, tx.state);
                                let instantiate = module.instantiate(&import_object);

                                match instantiate {
                                    Err(_) => {
                                        // ...
                                    }
                                    Ok(instance) => {
                                        let func = instance.dyn_func(&tx.func_name);

                                        match func {
                                            Err(_) => {
                                                // function not found
                                            }
                                            Ok(func) => {
                                                let args = [];
                                                let res = func.call(&args);

                                                match res {
                                                    Err(_) => {
                                                        // function execution failed
                                                    }
                                                    Ok(_) => {
                                                        // ...
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 1. Loads contract wasmer module `tx.contract`
                //  * if it's NOT in the compiled-modules-cache
                //      * Gets the wasm code from the `ENV::Store` (implements `CodeHashStore`)
                //      * Compile the module using `svm_compiler::compile_program(..)`
                //      * Store into the compiled-modules-cache
                //
                // 2. Validates that module has function `tx.FuncName` and that it can accept `tx.FuncArgs`
                //
                // 3. Builds the import object with `address = tx.Address` and `state = tx.State`
                //
                // 4. Instantiate wasm instance
                //
                // 5. Get the exported function `tx.FuncName`
                //
                // 6. Execute the function with input `tx.FuncArgs`
            }
        }
    };
}
