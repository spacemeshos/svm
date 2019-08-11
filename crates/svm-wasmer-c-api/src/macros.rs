/// Injects into the current file:
/// * `svm wasmer` instance C-API
/// * `svm wasmer` register C-API
/// * `svm vmcalls` (required by the implementations of the C-API functions)
#[macro_export]
macro_rules! include_svm_wasmer_c_api {
    ($KV:ident, $PS:ident, $PC:ident) => {
        use std::ffi::c_void;

        use wasmer_runtime::{imports, Ctx, ImportObject, Instance, Module};
        use wasmer_runtime_c_api::{
            error::{update_last_error, CApiError},
            export::wasmer_import_export_kind,
            import::{wasmer_import_object_extend, wasmer_import_object_t, wasmer_import_t},
            instance::{wasmer_instance_context_t, wasmer_instance_t},
            module::wasmer_module_t,
            wasmer_result_t,
        };
        use wasmer_runtime_core::{export::Export, import::Namespace};

        use crate::c_types::{svm_address_t, svm_contract_ctx_t, svm_receipt_t};
        use svm_wasmer::contract_ctx::ContractCtx;

        /// Injecting the `svm vmcalls` backed by page-cache `$PC` into this file
        include_wasmer_svm_vmcalls!($PC);

        /// Validates the deployed contract trnnsaction.
        /// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_deploy_contract_tx_validate(
            tx: *const u8,
            tx_len: u64,
        ) -> wasmer_result_t {
            unimplemented!()
        }

        /// Computes the new deployed contract's account address from the deployed transaction.
        /// The computation should be determinstic and take into account:
        ///
        /// * tag
        /// * author
        /// * admins
        /// * wasm code
        /// * deps revisions
        ///
        /// Address is returned via `addr` argument, and should be later dellocated
        /// using `svm_address_destroy` function (see file: `c_types.rs`).
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_deploy_contract_compute_addr(
            addr: *const *const svm_address_t,
            tx: *const u8,
            tx_len: u64,
        ) -> wasmer_result_t {
            unimplemented!()
        }

        /// Stores the new deployed contract under a database.
        /// Future transaction will reference the contract by it's account address.
        /// (see `wasmer_svm_contract_exec`).
        ///
        /// This function should be called after performing validation (see `wasmer_svm_deploy_contract_tx_validate`).
        ///
        /// * `addr` - The contract address. Should have been computed before.
        /// (see `wasmer_svm_deploy_contract_compute_addr`).
        ///
        /// * `tx` - The deployed contract on-the-wire transaction.
        /// * `tx_length` - The length of `tx`
        ///
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_deploy_contract_store(
            addr: *const svm_address_t,
            tx: *const u8,
            tx_length: u64,
        ) -> wasmer_result_t {
            unimplemented!()
        }

        /// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
        /// Mined transaction should be executed using `wasmer_svm_contract_prepare`.
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_contract_validate(
            addr: *const u8,
            state: *const u8,
            balance: *const u8,
            sender_addr: *const u8,
            sender_balance: *const u8,
            gas_left: u64,
            payload: *const u8,
            payload_len: u32,
            func_name: *const u8,
            func_name_len: u32,
        ) -> wasmer_result_t {
            unimplemented!()
        }

        /// Prepares a `context` object for executing a contract transaction.
        /// * `ctx`              - The result context object.
        /// * `addr`             - The account address of the contract.
        /// * `state`            - The hash-state of the contract storage.
        /// * `balance`          - The balance of the contract account.
        /// * `sender_addr`      - The account address of the transaction sender.
        /// * `sender_balance`   - The balance of the transaction sender.
        /// * `gas_left`         - How much Gas can be consumed while excuting the transaction.
        /// * `payload`          - A pointer to the args for the execution.
        /// * `payload_len`      - The length of the `payload`.
        /// * `func_name`        - A pointer to the name of the function to execute.
        /// * `func_name_len`    - The length of `func_name`.
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_contract_prepare(
            ctx: *mut *mut svm_contract_ctx_t,
            addr: *const u8,
            state: *const u8,
            balance: *const u8,
            sender_addr: *const u8,
            sender_balance: *const u8,
            gas_left: u64,
            payload: *const u8,
            payload_len: u32,
            func_name: *const u8,
            func_name_len: u32,
        ) -> wasmer_result_t {
            let contract_ctx = ContractCtx::from_raw(
                addr,
                state,
                balance,
                sender_addr,
                sender_balance,
                gas_left,
                payload,
                payload_len,
                func_name,
                func_name_len,
            );

            *ctx = Box::into_raw(Box::new(contract_ctx)) as *mut svm_contract_ctx_t;

            wasmer_result_t::WASMER_OK
        }

        /// Triggers an execution of an already deployed contract.
        ///
        /// `receipt` - The receipt of the contract execution.
        /// `ctx`     - The context object for the contract execution (see `wasmer_svm_contract_prepare`).
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_contract_exec(
            receipt: *mut *mut svm_receipt_t,
            ctx: *const *const svm_contract_ctx_t,
        ) -> wasmer_result_t {
            unimplemented!()
        }

        /// Compiles the wasm module using the `svm-compiler` (`wasmer` singlepass compiler with custom extensions)
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_compile(
            module: *mut *mut wasmer_module_t,
            wasm_bytes: *mut u8,
            wasm_bytes_len: u32,
        ) -> wasmer_result_t {
            let wasm: &[u8] = std::slice::from_raw_parts_mut(wasm_bytes, wasm_bytes_len as usize);
            let result = svm_compiler::compile_program(wasm);

            match result {
                Ok(wasmer_module) => {
                    let boxed_module = Box::new(wasmer_module);
                    *module = Box::into_raw(boxed_module) as *mut wasmer_module_t;
                    wasmer_result_t::WASMER_OK
                }
                Err(error) => {
                    update_last_error(error);
                    wasmer_result_t::WASMER_ERROR
                }
            }
        }

        /// Returns a raw pointer to the `wasmer svm` register's internal content
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_register_get(
            ctx: *const wasmer_instance_context_t,
            reg_idx: i32,
        ) -> *const u8 {
            use svm_wasmer::register::WasmerReg64;

            let wasmer_ctx: &Ctx = &*(ctx as *const Ctx);
            let reg: &mut WasmerReg64 = wasmer_ctx_reg!(wasmer_ctx, reg_idx, $PC);

            reg.as_ptr()
        }

        /// Copies `bytes_len` bytes from raw pointer `bytes` into `wasmer svm` register indexed `reg_idx`.
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_register_set(
            ctx: *const wasmer_instance_context_t,
            reg_idx: i32,
            bytes_ptr: *const u8,
            bytes_len: u8,
        ) {
            use svm_wasmer::register::WasmerReg64;

            let wasmer_ctx: &Ctx = &*(ctx as *const Ctx);
            let reg: &mut WasmerReg64 = wasmer_ctx_reg!(wasmer_ctx, reg_idx, $PC);

            reg.copy_from(bytes_ptr, bytes_len)
        }

        /// Gets the `node_data` field within the `svm context` (a.k.a `data` of the wasmer context).
        #[no_mangle]
        pub extern "C" fn wasmer_svm_instance_context_node_data_get(
            ctx: *const wasmer_instance_context_t,
        ) -> *const c_void {
            let wasmer_ctx: &Ctx = unsafe { &*(ctx as *const Ctx) };
            wasmer_data_node_data!(wasmer_ctx.data, $PC)
        }

        /// Creates a new `wasmer` import object.
        /// The import object will include imports of two flavors:
        /// * external vmcalls (i.e: node vmcalls)
        /// * internal vmcalls (i.e: register/storage/etc vmcalls)
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_import_object(
            raw_import_object: *mut *mut wasmer_import_object_t,
            addr_ptr: *const u8,
            max_pages: libc::c_int,
            max_page_slices: libc::c_int,
            node_data_ptr: *const c_void,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_uint,
        ) -> wasmer_runtime_c_api::wasmer_result_t {
            use wasmer_runtime::ImportObject;

            let state_gen = lazy_create_svm_state_gen!(
                node_data_ptr,
                svm_common::Address::from(addr_ptr),
                $KV,
                $PS,
                $PC,
                max_pages as usize,
                max_page_slices as usize
            );

            let mut import_object = ImportObject::new_with_data(state_gen);
            append_internal_imports(&mut import_object);

            *raw_import_object = cast_import_object_to_raw_ptr(import_object);
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
            ns.insert("mem_to_reg_copy", func!(mem_to_reg_copy));

            // ...
            // ...

            import_obj.register("svm", ns);
        }

        fn cast_import_object_to_raw_ptr(
            import_object: wasmer_runtime::ImportObject,
        ) -> *mut wasmer_import_object_t {
            let boxed_import_obj = Box::new(import_object);
            let import_obj_ptr: *mut _ = Box::into_raw(boxed_import_obj);

            import_obj_ptr as *mut _
        }
    };
}
