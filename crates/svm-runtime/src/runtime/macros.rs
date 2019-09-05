#[macro_export]
macro_rules! include_svm_runtime {
    ($pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $ENV: ty, $env_gen: expr) => {
        mod runtime {
            use $crate::runtime::ContractExecError;

            /// injects `vmcalls` module
            svm_runtime::include_svm_vmcalls!($PC);

            use svm_common::{Address, State};

            use svm_contract::{
                env::ContractEnv,
                error::{ContractBuildError, TransactionBuildError},
                traits::ContractStore,
                transaction::Transaction,
                wasm::Contract,
            };

            #[inline(always)]
            pub fn contract_build(bytes: &[u8]) -> Result<Contract, ContractBuildError> {
                <$ENV as ContractEnv>::build_contract(bytes)
            }

            #[inline(always)]
            pub fn contract_deploy_validate(contract: &Contract) -> Result<(), ContractBuildError> {
                // TODO:
                // validate the `wasm`. should use the `deterministic` feature of `wasmparser`.
                // (avoiding floats etc.)

                Ok(())
            }

            #[inline(always)]
            pub fn contract_store(contract: &Contract) {
                let mut env = $env_gen();
                env.store_contract(&contract)
            }

            #[inline(always)]
            pub fn transaction_build(bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
                <$ENV as ContractEnv>::build_transaction(bytes)
            }

            pub fn contract_exec(
                tx: &Transaction,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<State, ContractExecError> {
                let mut env = $env_gen();
                let contract = contract_load(tx, &mut env)?;
                let module = contract_compile(&contract)?;
                let mut instance = module_instantiate(&contract, &module, import_object)?;
                let args = prepare_args_and_memory(tx, &mut instance);
                let func = get_exported_func(&instance, &tx.func_name)?;

                let res = func.call(&args);

                match res {
                    Err(_) => Err(ContractExecError::ExecFailed),
                    Ok(_) => {
                        let storage = get_instance_svm_storage_mut(&mut instance);
                        let state = storage.commit();

                        Ok(state)
                    }
                }
            }

            pub fn import_object_create(
                addr: Address,
                state: State,
                node_data: *const std::ffi::c_void,
                opts: $crate::opts::Opts,
            ) -> wasmer_runtime::ImportObject {
                use wasmer_runtime::{func, ImportObject};

                let max_pages = opts.max_pages;
                let wrapped_pages_storage_gen = move || $pages_storage_gen(addr, state, max_pages);

                let state_gen = svm_runtime::lazy_create_svm_state_gen!(
                    node_data,
                    wrapped_pages_storage_gen,
                    $page_cache_ctor,
                    $PC,
                    opts
                );

                let mut import_object = ImportObject::new_with_data(state_gen);

                let mut ns = wasmer_runtime_core::import::Namespace::new();

                // storage
                ns.insert("mem_to_reg_copy", func!(vmcalls::mem_to_reg_copy));
                ns.insert("reg_to_mem_copy", func!(vmcalls::reg_to_mem_copy));
                ns.insert("storage_read_to_reg", func!(vmcalls::storage_read_to_reg));
                ns.insert("storage_read_to_mem", func!(vmcalls::storage_read_to_mem));
                ns.insert(
                    "storage_write_from_mem",
                    func!(vmcalls::storage_write_from_mem),
                );
                ns.insert(
                    "storage_write_from_reg",
                    func!(vmcalls::storage_write_from_reg),
                );

                // register
                ns.insert("reg_read_le_i64", func!(vmcalls::reg_read_le_i64));
                ns.insert("reg_write_le_i64", func!(vmcalls::reg_write_le_i64));

                import_object.register("svm", ns);

                import_object
            }

            fn contract_load(
                tx: &Transaction,
                env: &mut $ENV,
            ) -> Result<Contract, ContractExecError> {
                let store = env.get_store();

                match store.load(tx.contract) {
                    None => Err(ContractExecError::NotFound(tx.contract)),
                    Some(contract) => Ok(contract),
                }
            }

            fn contract_compile(
                contract: &Contract,
            ) -> Result<wasmer_runtime::Module, ContractExecError> {
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
                contract: &Contract,
                module: &wasmer_runtime::Module,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<wasmer_runtime::Instance, ContractExecError> {
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
            ) -> Result<wasmer_runtime::DynFunc<'a>, ContractExecError> {
                let func = instance.dyn_func(func_name);

                match func {
                    Err(_) => Err(ContractExecError::FuncNotFound(func_name.to_string())),
                    Ok(func) => Ok(func),
                }
            }

            fn prepare_args_and_memory(
                tx: &Transaction,
                instance: &mut wasmer_runtime::Instance,
            ) -> Vec<wasmer_runtime::Value> {
                use svm_contract::wasm::{WasmArgValue, WasmIntType};
                use wasmer_runtime::Value;

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

                wasmer_args
            }

            #[inline(always)]
            fn get_instance_svm_storage_mut(
                instance: &mut wasmer_runtime::Instance,
            ) -> &mut svm_storage::PageSliceCache<$PC> {
                let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();

                $crate::wasmer_data_storage!(wasmer_ctx.data, $PC)
            }
        }
    };
}
