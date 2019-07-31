/// Injects into the current file:
/// * `svm wasmer` instance C-API
/// * `svm wasmer` register C-API
/// * `svm vmcalls` (required by the implementations of the C-API functions)
#[macro_export]
macro_rules! include_svm_wasmer_c_api {
    ($KV:ident, $PS:ident, $PC:ident) => {
        use std::ffi::c_void;

        use crate::import::wasmer_import_object_t;

        use wasmer_runtime::{imports, Ctx, ImportObject, Instance, Module};
        use wasmer_runtime_c_api::{
            error::{update_last_error, CApiError},
            export::wasmer_import_export_kind,
            import::wasmer_import_t,
            instance::{wasmer_instance_context_t, wasmer_instance_t},
            module::wasmer_module_t,
            wasmer_result_t,
        };
        use wasmer_runtime_core::{export::Export, import::Namespace};

        /// Injecting the `svm vmcalls` backed by page-cache `$PC` into this file
        include_wasmer_svm_vmcalls!($PC);

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

        /// Given:
        /// * A prepared `wasmer svm` import-object
        /// * A compiled wasmer module
        ///
        /// Instantiates a wasmer instance
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_module_instantiate(
            instance_ptr_ptr: *mut *mut wasmer_instance_t,
            module: *const wasmer_module_t,
            import_object: *const wasmer_import_object_t,
        ) -> wasmer_runtime_c_api::wasmer_result_t {
            let import_object: &ImportObject = &*(import_object as *const ImportObject);
            let module: &Module = &*(module as *const Module);

            let new_instance: Instance = match module.instantiate(import_object) {
                Ok(instance) => instance,
                Err(error) => {
                    update_last_error(error);
                    return wasmer_result_t::WASMER_ERROR;
                }
            };
            *instance_ptr_ptr = Box::into_raw(Box::new(new_instance)) as *mut wasmer_instance_t;

            return wasmer_result_t::WASMER_OK;
        }

        /// Creates a new `wasmer` import object.
        /// The import object will include imports of two flavors:
        /// * external vmcalls (i.e: node vmcalls)
        /// * internal vmcalls (i.e: register/storage/etc vmcalls)
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_import_object(
            imprt_obj_ptr_ptr: *mut *mut wasmer_import_object_t,
            addr_ptr: *const u8,
            max_pages: libc::c_int,
            max_page_slices: libc::c_int,
            node_data_ptr: *const c_void,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_int,
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

            let mut import_obj = ImportObject::new_with_data(state_gen);

            append_internal_imports(&mut import_obj);

            let _res = append_external_imports(&mut import_obj, imports, imports_len);
            // TODO: assert result
            // if res != wasmer_result_t::WASMER_OK {
            //     return res;
            // }

            *imprt_obj_ptr_ptr = cast_import_obj_to_ptr(import_obj);

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

        fn cast_import_obj_to_ptr(
            import_obj: wasmer_runtime::ImportObject,
        ) -> *mut wasmer_import_object_t {
            let boxed_import_obj = Box::new(import_obj);
            let import_obj_ptr: *mut _ = Box::into_raw(boxed_import_obj);

            import_obj_ptr as *mut _
        }
    };
}
