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

        /// Injecting the `svm vmcalls` backed by page-cache `$PC` into this file
        include_wasmer_svm_vmcalls!($PC);

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

            *raw_import_object = cast_import_obj_to_ptr(import_object);
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

        fn cast_import_obj_to_ptr(
            import_object: wasmer_runtime::ImportObject,
        ) -> *mut wasmer_import_object_t {
            let boxed_import_obj = Box::new(import_object);
            let import_obj_ptr: *mut _ = Box::into_raw(boxed_import_obj);

            import_obj_ptr as *mut _
        }
    };
}
