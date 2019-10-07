/// Injects into the current file `svm runtime C-API`
#[macro_export]
macro_rules! include_svm_runtime_c_api {
    ($pages_storage_gen: expr, $page_cache_ctor: expr, $PC: path, $ENV: path, $env_gen: expr) => {
        /// Injects `runtime` module into this file
        svm_runtime::include_svm_runtime!(
            $pages_storage_gen,
            $page_cache_ctor,
            $PC,
            $ENV,
            $env_gen
        );

        use svm_common::{Address, State};
        use svm_contract::transaction::Transaction;

        use svm_runtime::register::SvmReg;
        use svm_runtime::runtime::Receipt;

        use crate::c_types::{svm_contract_t, svm_receipt_t, svm_transaction_t};

        use log::{debug, error, trace};
        use std::ffi::c_void;

        use wasmer_runtime::{Ctx, ImportObject};
        use wasmer_runtime_c_api::{
            error::update_last_error,
            import::{wasmer_import_object_extend, wasmer_import_object_t, wasmer_import_t},
            instance::wasmer_instance_context_t,
            value::wasmer_value_t,
            wasmer_result_t,
        };

        macro_rules! into_raw {
            ($obj: expr, $raw_type: ident) => {{
                let boxed_obj = Box::new($obj);
                let raw_obj_ptr: *mut _ = Box::into_raw(boxed_obj);

                raw_obj_ptr as *mut $raw_type
            }};
        }

        macro_rules! cast_to_rust_type {
            ($raw_obj: expr, $ty: path) => {{
                &*($raw_obj as *const $ty)
            }};
        }

        /// Builds an instance of `svm_contract_t`.
        /// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_contract_build(
            raw_contract: *mut *mut svm_contract_t,
            raw_bytes: *const c_void,
            raw_bytes_len: u64,
        ) -> wasmer_result_t {
            debug!("`svm_contract_build start`");

            let bytes = std::slice::from_raw_parts(raw_bytes as *const u8, raw_bytes_len as usize);
            let result = runtime::contract_build(&bytes);

            match result {
                Ok(contract) => {
                    *raw_contract = into_raw!(contract, svm_contract_t);
                    debug!("`svm_contract_build returns `WASMER_OK`");
                    wasmer_result_t::WASMER_OK
                }
                Err(err) => {
                    update_last_error(err);
                    error!("`svm_contract_build returns `WASMER_ERROR`");
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Computes the contract to-be-deployed acccunt address and retures a pointer to it
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_contract_compute_address(
            raw_contract: *const svm_contract_t,
        ) -> *const c_void {
            debug!("`svm_contract_compute_address`");

            let contract = cast_to_rust_type!(raw_contract, svm_contract::wasm::Contract);

            let addr = runtime::contract_compute_address(contract);
            let addr = Box::leak(Box::new(addr));

            addr.as_ptr() as *const c_void
        }

        /// Stores the new deployed contract under a database.
        /// Future transaction will reference the contract by it's account address.
        /// (see `svm_transaction_exec`)
        ///
        /// This function should be called after performing validation.
        ///
        /// * `raw_contract` - The wasm contract to be stored
        ///
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_contract_store(
            raw_contract: *const svm_contract_t,
            raw_addr: *const c_void,
        ) -> wasmer_result_t {
            debug!("`svm_contract_store` start");

            let contract = cast_to_rust_type!(raw_contract, svm_contract::wasm::Contract);
            let addr = Address::from(raw_addr);
            runtime::contract_store(contract, &addr);

            debug!("`svm_contract_build returns `WASMER_OK`");

            wasmer_result_t::WASMER_OK
        }

        /// Builds an instance of `svm_transaction_t`.
        /// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_transaction_build(
            raw_tx: *mut *mut svm_transaction_t,
            raw_bytes: *const c_void,
            raw_bytes_len: u64,
        ) -> wasmer_result_t {
            let bytes: &[u8] =
                std::slice::from_raw_parts(raw_bytes as *const u8, raw_bytes_len as usize);
            let result = runtime::transaction_build(bytes);

            match result {
                Ok(tx) => {
                    *raw_tx = into_raw!(tx, svm_transaction_t);
                    debug!("`svm_contract_build returns `WASMER_OK`");
                    wasmer_result_t::WASMER_OK
                }
                Err(error) => {
                    update_last_error(error);
                    error!("`svm_contract_build returns `WASMER_ERROR`");
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Triggers a transaction execution of an already deployed contract.
        ///
        /// `receipt` - The receipt of the contract execution.
        /// `tx`      - The transaction to execute.
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_transaction_exec(
            raw_receipt: *mut *mut svm_receipt_t,
            raw_tx: *const svm_transaction_t,
            raw_import_object: *const wasmer_import_object_t,
        ) -> wasmer_result_t {
            debug!("`svm_transaction_exec` start");

            let tx = cast_to_rust_type!(raw_tx, Transaction);
            let import_object = cast_to_rust_type!(raw_import_object, ImportObject);

            let receipt = runtime::contract_exec(tx.clone(), import_object);
            *raw_receipt = into_raw!(receipt, svm_receipt_t);

            debug!("`svm_contract_build returns `WASMER_OK`");

            wasmer_result_t::WASMER_OK
        }

        /// Returns a raw pointer to the `wasmer svm` register's internal content
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_register_get(
            raw_ctx: *const wasmer_instance_context_t,
            reg_bits: i32,
            reg_idx: i32,
        ) -> *const c_void {
            debug!("`svm_register_get` register `{}:{}`", reg_bits, reg_idx);

            let wasmer_ctx: &Ctx = cast_to_rust_type!(raw_ctx, Ctx);
            let reg: &mut SvmReg = svm_runtime::wasmer_ctx_reg!(wasmer_ctx, reg_bits, reg_idx, $PC);

            // having `c_void` instead of `u8` in the function's signature
            // makes the integration with `cgo` easier.
            reg.as_ptr() as *mut u8 as *mut c_void
        }

        /// Copies `bytes_len` bytes from raw pointer `bytes` into `wasmer svm` register indexed `reg_idx`.
        #[no_mangle]
        pub unsafe extern "C" fn svm_register_set(
            raw_ctx: *const wasmer_instance_context_t,
            reg_bits: i32,
            reg_idx: i32,
            bytes: *const c_void,
            bytes_len: u8,
        ) {
            debug!("`svm_register_set` register `{}:{}`", reg_bits, reg_idx);

            let wasmer_ctx: &Ctx = cast_to_rust_type!(raw_ctx, Ctx);
            let reg: &mut SvmReg = svm_runtime::wasmer_ctx_reg!(wasmer_ctx, reg_bits, reg_idx, $PC);

            // having `c_void` instead of `u8` in the function's signature
            // makes the integration with `cgo` easier.
            let bytes = bytes as *const u8;
            reg.copy_from(bytes, bytes_len)
        }

        /// Gets the `node_data` field within the `svm context` (a.k.a `data` of the wasmer context).
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_instance_context_node_data_get(
            raw_ctx: *const wasmer_instance_context_t,
        ) -> *const c_void {
            trace!("`svm_instance_context_node_data_get`");

            let wasmer_ctx: &Ctx = cast_to_rust_type!(raw_ctx, Ctx);
            svm_runtime::wasmer_data_node_data!(wasmer_ctx.data, $PC)
        }

        /// Creates a new `wasmer` import object.
        /// The import object will include imports of two flavors:
        /// * external vmcalls (i.e: node vmcalls)
        /// * internal vmcalls (i.e: register/storage/etc vmcalls)
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_import_object(
            raw_import_object: *mut *mut wasmer_import_object_t,
            raw_addr: *const c_void,
            raw_state: *const c_void,
            raw_max_pages: libc::c_int,
            raw_max_page_slices: libc::c_int,
            node_data: *const c_void,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_uint,
        ) -> wasmer_result_t {
            debug!("`svm_import_object` start");

            let addr = Address::from(raw_addr);
            let state = State::from(raw_state);

            let opts = svm_runtime::opts::Opts {
                max_pages: raw_max_pages as usize,
                max_pages_slices: raw_max_page_slices as usize,
            };

            let import_object = runtime::import_object_create(addr, state, node_data, opts);

            *raw_import_object = into_raw!(import_object, wasmer_import_object_t);

            let _res = wasmer_import_object_extend(*raw_import_object, imports, imports_len);
            // TODO: assert result
            // if result != wasmer_result_t::WASMER_OK {
            //     return result;
            // }

            debug!("`svm_import_object` returns `WASMER_OK`");

            wasmer_result_t::WASMER_OK
        }

        /// Returns the receipt outcome (`true` for success and `false` otherwise)
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_receipt_status(raw_receipt: *const svm_receipt_t) -> bool {
            let receipt = cast_to_rust_type!(raw_receipt, Receipt);
            debug!("`svm_receipt_status` status={}", receipt.success);

            receipt.success
        }

        /// Returns the transaction execution results (wasm array).
        /// Should be called only after verifying that the transaction succeeded.
        /// Will panic when called for a failed transaction.
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_receipt_results(
            raw_receipt: *const svm_receipt_t,
            results: *mut *mut wasmer_value_t,
            results_len: *mut u32,
        ) {
            debug!("`svm_receipt_results`");

            let receipt = cast_to_rust_type!(raw_receipt, Receipt);

            if receipt.success {
                let mut c_results = Vec::with_capacity(*results_len as usize);

                for value in receipt.results.iter() {
                    let c_value = wasmer_value_t::from(value.clone());
                    c_results.push(c_value);
                }

                // TODO: free `c_results` memory after usage
                let c_results: &mut Vec<wasmer_value_t> = Box::leak(Box::new(c_results));

                *results = c_results.as_mut_ptr();
                *results_len = receipt.results.len() as u32;
            } else {
                let msg = "method not allowed to be called when transaction execution failed";
                error!("{}", msg);
                panic!("{}", msg);
            }
        }

        /// Returns the `receipt` error in transaction failed
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_receipt_error(raw_receipt: *const svm_receipt_t) {
            let receipt = cast_to_rust_type!(raw_receipt, Receipt);

            if let Some(ref _e) = receipt.error {
                // TODO: implement `std::error::Error` for `svm_runtime::runtime::error::ContractExecError`
                // update_last_error(e);
            }
        }

        /// Returns a pointer to the new state of the contract account.
        #[must_use]
        #[no_mangle]
        pub unsafe extern "C" fn svm_receipt_new_state(
            raw_receipt: *const svm_receipt_t,
        ) -> *const u8 {
            let receipt = cast_to_rust_type!(raw_receipt, Receipt);

            if receipt.success {
                let state = receipt.new_state.as_ref().unwrap();
                state.as_ptr()
            } else {
                panic!("method not allowed to be called when transaction execution failed");
            }
        }
    };
}
