/// Injects a `svm` runtime into current file.
///
/// * `pages_storage_gen` - a function generating a new `svm_storage::traits::PagesStorage`
///
/// * `page_cache_ctor` - a function generating a new `svm_storage::traits::PageCache`
///     wrapping the `PagesStorage` generated by `pages_storage_gen` above.
///
/// * `ENV` - the environment type. This type implements trait `svm_contract::ContractEnv`
///
/// * `env_gen` - a function generating an environment. The environment will be of type `ENV` above.
#[macro_export]
macro_rules! include_svm_runtime {
    ($pages_storage_gen: expr, $page_cache_ctor: expr, $ENV: path, $env_gen: expr) => {
        mod runtime {
            use log::{debug, error, info};

            use $crate::runtime::{ContractExecError, Receipt};

            /// Injects `vmcalls` module into the current file
            svm_runtime::include_svm_vmcalls!();

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
                debug!("runtime `contract_build`");

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
            pub fn contract_compute_address(contract: &Contract) -> Address {
                debug!("runtime `contract_compute_address`");

                <$ENV as ContractEnv>::compute_address(contract)
            }

            #[inline(always)]
            pub fn contract_store(contract: &Contract, addr: &Address) {
                debug!("runtime `contract_store`");

                let mut env = $env_gen();
                env.store_contract(contract, addr);
            }

            #[inline(always)]
            pub fn transaction_build(bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
                debug!("runtime `transaction_build`");

                <$ENV as ContractEnv>::build_transaction(bytes)
            }

            pub fn contract_exec(
                tx: Transaction,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Receipt {
                debug!("runtime `contract_exec`");

                let receipt = match do_contract_exec(&tx, import_object) {
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

                debug!("receipt: {:?}", receipt);

                receipt
            }

            pub fn import_object_create(
                addr: Address,
                state: State,
                node_data: *const std::ffi::c_void,
                opts: $crate::opts::Opts,
            ) -> wasmer_runtime::ImportObject {
                use svm_runtime::ctx_data_wrapper::SvmCtxDataWrapper;
                use wasmer_runtime::{func, ImportObject};

                debug!(
                    "runtime `import_object_create` address={:?}, state={:?}, opts={:?}",
                    addr, state, opts
                );

                let wrapped_pages_storage_gen =
                    move || $pages_storage_gen(addr.clone(), state.clone(), opts.max_pages);

                let wrapped_data = SvmCtxDataWrapper::new(node_data);

                let state_gen = svm_runtime::lazy_create_svm_state_gen!(
                    wrapped_data,
                    wrapped_pages_storage_gen,
                    $page_cache_ctor,
                    opts
                );

                let mut import_object = ImportObject::new_with_data(state_gen);

                let mut ns = wasmer_runtime_core::import::Namespace::new();

                // storage vmcalls
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

                // register vmcalls
                ns.insert("reg_replace_byte", func!(vmcalls::reg_replace_byte));
                ns.insert("reg_read_be_i64", func!(vmcalls::reg_read_be_i64));
                ns.insert("reg_write_be_i64", func!(vmcalls::reg_write_be_i64));

                import_object.register("svm", ns);

                import_object
            }

            #[inline(always)]
            fn do_contract_exec(
                tx: &Transaction,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<(State, Vec<wasmer_runtime::Value>), ContractExecError> {
                let mut env = $env_gen();

                let contract = contract_load(tx, &mut env)?;
                let module = contract_compile(&contract, &tx.contract)?;
                let mut instance = instantiate(&contract, &tx.contract, &module, import_object)?;
                let args = prepare_args_and_memory(tx, &mut instance);
                let func = get_exported_func(&instance, &tx.func_name)?;

                match func.call(&args) {
                    Err(e) => Err(ContractExecError::ExecFailed),
                    Ok(results) => {
                        let storage = get_instance_svm_storage_mut(&mut instance);
                        let state = storage.commit();
                        Ok((state, results))
                    }
                }
            }

            fn contract_load(
                tx: &Transaction,
                env: &mut $ENV,
            ) -> Result<Contract, ContractExecError> {
                info!("runtime `contract_load`");

                let store = env.get_store();

                match store.load(&tx.contract) {
                    None => Err(ContractExecError::NotFound(tx.contract.clone())),
                    Some(contract) => Ok(contract),
                }
            }

            fn contract_compile(
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

            fn instantiate(
                contract: &Contract,
                addr: &Address,
                module: &wasmer_runtime::Module,
                import_object: &wasmer_runtime::ImportObject,
            ) -> Result<wasmer_runtime::Instance, ContractExecError> {
                info!("runtime `instantiate` (wasmer module instantiate)");

                let instantiate = module.instantiate(import_object);

                match instantiate {
                    Err(e) => Err(ContractExecError::InstantiationFailed(addr.clone())),
                    Ok(instance) => Ok(instance),
                }
            }

            fn get_exported_func<'a>(
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
                instance: &mut wasmer_runtime::Instance,
            ) -> &mut svm_storage::ContractStorage {
                let wasmer_ctx: &mut wasmer_runtime::Ctx = instance.context_mut();

                $crate::wasmer_data_storage!(wasmer_ctx.data)
            }
        }
    };
}
