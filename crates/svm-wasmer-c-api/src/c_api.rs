/// Injects into the current file:
/// * `svm wasmer` instance C-API
/// * `svm wasmer` register C-API
/// * `svm vmcalls` (required by the implementations of the C-API functions)
#[macro_export]
macro_rules! include_svm_wasmer_c_api {
    ($KV:ident, $PS:ident, $PC:ident) => {
        use std::ffi::c_void;

        use wasmer_runtime::Ctx;
        use wasmer_runtime_c_api::error::{update_last_error, CApiError};
        use wasmer_runtime_core::import::Namespace;

        /// Injecting the `svm vmcalls` backed by page-cache `$PC` into this file
        include_wasmer_svm_vmcalls!($PC);

        /// Returns a raw pointer to the `wasmer svm` register's internal content
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_register_ptr(
            ctx: *const wasmer_instance_context_t,
            reg_idx: i32,
        ) -> *const u8 {
            use svm_wasmer::register::WasmerReg64;

            let wasmer_ctx: &Ctx = unsafe { &*(ctx as *const Ctx) };
            let reg: &mut WasmerReg64 = wasmer_ctx_reg!(wasmer_ctx, reg_idx, $PC);

            reg.as_ptr()
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
            import_object_ptr: *mut *mut c_void,
            addr_ptr: *const u8,
            max_pages: libc::c_int,
            max_page_slices: libc::c_int,
            node_data_ptr: *const c_void,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_int,
        ) -> wasmer_runtime_c_api::wasmer_result_t {
            use svm_common::Address;
            use wasmer_runtime::ImportObject;

            let state_gen = lazy_create_svm_state_gen!(
                node_data_ptr,
                Address::from(addr_ptr),
                $KV,
                $PS,
                $PC,
                max_pages as usize,
                max_page_slices as usize
            );

            let mut import_obj = ImportObject::new_with_data(state_gen);

            append_internal_imports(&mut import_obj);

            let _res = append_external_imports(&mut import_obj, imports, imports_len);
            // TODO: assert result
            // if res != wasmer_result_t::WASMER_OK {
            //     return res;
            // }

            *import_object_ptr = cast_import_obj_to_ptr(import_obj);

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

        unsafe fn append_external_imports(
            import_object: &mut wasmer_runtime::ImportObject,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_int,
        ) -> wasmer_result_t {
            use std::collections::HashMap;
            use std::slice;

            /// original code has been takes from `wasmer_instantiate` located at:
            /// https://github.com/wasmerio/wasmer/blob/master/lib/runtime-c-api/src/instance.rs
            let imports: &[wasmer_import_t] = slice::from_raw_parts(imports, imports_len as usize);
            let mut namespaces = HashMap::new();

            for import in imports {
                let module_name = slice::from_raw_parts(
                    import.module_name.bytes,
                    import.module_name.bytes_len as usize,
                );
                let module_name = if let Ok(s) = std::str::from_utf8(module_name) {
                    s
                } else {
                    update_last_error(CApiError {
                        msg: "error converting module name to string".to_string(),
                    });
                    return wasmer_result_t::WASMER_ERROR;
                };
                let import_name = slice::from_raw_parts(
                    import.import_name.bytes,
                    import.import_name.bytes_len as usize,
                );
                let import_name = if let Ok(s) = std::str::from_utf8(import_name) {
                    s
                } else {
                    update_last_error(CApiError {
                        msg: "error converting import_name to string".to_string(),
                    });
                    return wasmer_result_t::WASMER_ERROR;
                };

                let namespace = namespaces.entry(module_name).or_insert_with(Namespace::new);

                let export = match import.tag {
                    wasmer_import_export_kind::WASM_FUNCTION => {
                        let func_export = import.value.func as *mut Export;
                        (&*func_export).clone()
                    }
                    _ => unreachable!(),
                };
                namespace.insert(import_name, export);
            }

            for (module_name, namespace) in namespaces.into_iter() {
                import_object.register(module_name, namespace);
            }

            wasmer_result_t::WASMER_OK
        }

        fn cast_import_obj_to_ptr(import_obj: wasmer_runtime::ImportObject) -> *mut c_void {
            let boxed_import_obj = Box::new(import_obj);
            let import_obj_ptr: *mut _ = Box::into_raw(boxed_import_obj);

            import_obj_ptr as *mut c_void
        }
    };
}
