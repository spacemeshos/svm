///
/// Injects into the current file:
/// * `svm wasmer` instance C-API
/// * `svm wasmer` register C-API
/// * `svm vmcalls` (required by the implementations of the C-API functions)
///
#[macro_export]
macro_rules! include_svm_wasmer_c_api {
    ($pages_storage_gen: expr, $PC: ident, $ENV: path, $env_gen: expr) => {
        /// Injecting the `svm runtime` backed by PageCache `$PC` into this file
        include_svm_runtime!($PC, $ENV, $env_gen);

        /// Injecting `svm_wasmer` macros
        use svm_wasmer::*;

        use svm_common::{Address, State};
        use svm_contract::transaction::Transaction;
        use svm_wasmer::register::SvmReg;

        use crate::c_types::{
            svm_address_t, svm_receipt_t, svm_transaction_t, svm_wasm_contract_t,
        };

        use std::ffi::c_void;

        use wasmer_runtime::{Ctx, ImportObject};
        use wasmer_runtime_c_api::{
            error::update_last_error,
            import::{wasmer_import_object_extend, wasmer_import_object_t, wasmer_import_t},
            instance::wasmer_instance_context_t,
            module::wasmer_module_t,
            wasmer_result_t,
        };
        use wasmer_runtime_core::import::Namespace;

        macro_rules! cast_obj_to_raw_ptr {
            ($obj: expr, $raw_type: ident) => {{
                let boxed_obj = Box::new($obj);
                let raw_obj_ptr: *mut _ = Box::into_raw(boxed_obj);

                raw_obj_ptr as *mut $raw_type
            }};
        }

        macro_rules! from_raw {
            ($raw_obj: expr, $ty: path) => {{
                &*($raw_obj as *const $ty)
            }};
        }

        /// Builds an instance of `svm_wasm_contract_t`.
        /// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_contract_build(
            raw_contract: *mut *mut svm_wasm_contract_t,
            raw_bytes: *const u8,
            raw_bytes_len: u64,
        ) -> wasmer_result_t {
            let bytes = std::slice::from_raw_parts(raw_bytes, raw_bytes_len as usize);
            let res = runtime::contract_build(&bytes);

            match res {
                Ok(contract) => {
                    *raw_contract = cast_obj_to_raw_ptr!(contract, svm_wasm_contract_t);
                    wasmer_result_t::WASMER_OK
                }
                Err(err) => {
                    update_last_error(err);
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Stores the new deployed contract under a database.
        /// Future transaction will reference the contract by it's account address.
        /// (see `wasmer_svm_transaction_exec`)
        ///
        /// This function should be called after performing validation.
        ///
        /// * `raw_contract` - The wasm contract to be stored
        ///
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_contract_store(
            raw_contract: *const svm_wasm_contract_t,
        ) -> wasmer_result_t {
            let contract = from_raw!(raw_contract, svm_contract::wasm::Contract);
            runtime::contract_store(&contract);

            wasmer_result_t::WASMER_OK
        }

        /// Builds an instance of `svm_transaction_t`.
        /// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_transaction_build(
            raw_tx: *mut *mut svm_transaction_t,
            raw_bytes: *mut u8,
            raw_bytes_len: u64,
        ) -> wasmer_result_t {
            let bytes: &[u8] = std::slice::from_raw_parts_mut(raw_bytes, raw_bytes_len as usize);
            let result = runtime::transaction_build(bytes);

            match result {
                Ok(tx) => {
                    *raw_tx = cast_obj_to_raw_ptr!(tx, svm_transaction_t);
                    wasmer_result_t::WASMER_OK
                }
                Err(error) => {
                    update_last_error(error);
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Compiles the wasm module using the `svm-compiler` (`wasmer` singlepass compiler with custom extensions)
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_compile(
            raw_module: *mut *mut wasmer_module_t,
            bytes: *mut u8,
            bytes_len: u32,
        ) -> wasmer_result_t {
            let raw_bytes = std::slice::from_raw_parts_mut(bytes, bytes_len as usize);
            let result = svm_compiler::compile_program(raw_bytes);

            match result {
                Ok(module) => {
                    *raw_module = cast_obj_to_raw_ptr!(module, wasmer_module_t);
                    wasmer_result_t::WASMER_OK
                }
                Err(error) => {
                    update_last_error(error);
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Triggers a transaction execution of an already deployed contract.
        ///
        /// `receipt` - The receipt of the contract execution.
        /// `tx`      - The transaction to execute.
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_transaction_exec(
            receipt: *mut *mut svm_receipt_t,
            raw_tx: *const svm_transaction_t,
            raw_import_object: *const wasmer_import_object_t,
        ) -> wasmer_result_t {
            let tx = from_raw!(raw_tx, Transaction);
            let import_object = from_raw!(raw_import_object, ImportObject);

            match runtime::contract_exec(tx, import_object) {
                Ok(_) => wasmer_result_t::WASMER_OK,
                Err(error) => {
                    update_last_error(error);
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Returns a raw pointer to the `wasmer svm` register's internal content
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_register_get(
            raw_ctx: *const wasmer_instance_context_t,
            reg_bits: i32,
            reg_idx: i32,
        ) -> *const c_void {
            let wasmer_ctx: &Ctx = from_raw!(raw_ctx, Ctx);
            let reg: &mut SvmReg = wasmer_ctx_reg!(wasmer_ctx, reg_bits, reg_idx, $PC);

            // having `c_void` instead of `u8` in the function's signature
            // makes the integration with `cgo` easier.
            reg.as_ptr() as *mut u8 as *mut c_void
        }

        /// Copies `bytes_len` bytes from raw pointer `bytes` into `wasmer svm` register indexed `reg_idx`.
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_register_set(
            raw_ctx: *const wasmer_instance_context_t,
            reg_bits: i32,
            reg_idx: i32,
            bytes: *const c_void,
            bytes_len: u8,
        ) {
            let wasmer_ctx: &Ctx = from_raw!(raw_ctx, Ctx);
            let reg: &mut SvmReg = wasmer_ctx_reg!(wasmer_ctx, reg_bits, reg_idx, $PC);

            // having `c_void` instead of `u8` in the function's signature
            // makes the integration with `cgo` easier.
            let bytes = bytes as *const u8;
            reg.copy_from(bytes, bytes_len)
        }

        /// Gets the `node_data` field within the `svm context` (a.k.a `data` of the wasmer context).
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_instance_context_node_data_get(
            raw_ctx: *const wasmer_instance_context_t,
        ) -> *const c_void {
            let wasmer_ctx: &Ctx = from_raw!(raw_ctx, Ctx);
            wasmer_data_node_data!(wasmer_ctx.data, $PC)
        }

        /// Creates a new `wasmer` import object.
        /// The import object will include imports of two flavors:
        /// * external vmcalls (i.e: node vmcalls)
        /// * internal vmcalls (i.e: register/storage/etc vmcalls)
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_import_object(
            raw_import_object: *mut *mut wasmer_import_object_t,
            raw_addr: *const c_void,
            raw_state: *const c_void,
            raw_max_pages: libc::c_int,
            raw_max_page_slices: libc::c_int,
            node_data: *const c_void,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_uint,
        ) -> wasmer_result_t {
            let max_pages: u32 = raw_max_pages as u32;
            let max_page_slices: u32 = raw_max_page_slices as u32;

            // having `c_void` instead of `u8` in the function's signature
            // makes the integration with `cgo` easier.
            let wrapped_pages_storage_gen = move || {
                let addr = Address::from(raw_addr as *const u8);
                let state = State::from(raw_state as *const u8);

                $pages_storage_gen(addr, state, max_pages)
            };

            let state_gen = lazy_create_svm_state_gen!(
                node_data,
                wrapped_pages_storage_gen,
                $PC,
                max_pages as usize,
                max_page_slices as usize
            );

            let mut import_object = ImportObject::new_with_data(state_gen);
            append_internal_imports(&mut import_object);

            *raw_import_object = cast_obj_to_raw_ptr!(import_object, wasmer_import_object_t);
            let _res = wasmer_import_object_extend(*raw_import_object, imports, imports_len);
            // TODO: assert result
            // if res != wasmer_result_t::WASMER_OK {
            //     return res;
            // }

            wasmer_result_t::WASMER_OK
        }

        fn append_internal_imports(import_obj: &mut wasmer_runtime::ImportObject) {
            use wasmer_runtime::func;

            let mut ns = Namespace::new();

            // storage
            ns.insert("mem_to_reg_copy", func!(mem_to_reg_copy));
            ns.insert("reg_to_mem_copy", func!(reg_to_mem_copy));
            ns.insert("storage_read_to_reg", func!(storage_read_to_reg));
            ns.insert("storage_read_to_mem", func!(storage_read_to_mem));
            ns.insert("storage_write_from_mem", func!(storage_write_from_mem));
            ns.insert("storage_write_from_reg", func!(storage_write_from_reg));

            // register
            ns.insert("reg_read_le_i64", func!(reg_read_le_i64));
            ns.insert("reg_write_le_i64", func!(reg_write_le_i64));

            import_obj.register("svm", ns);
        }
    };
}
