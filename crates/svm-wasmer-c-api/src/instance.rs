/// Inject to the current file:
/// * `svm wasmer` instance API (`wasmer_svm_import_object` etc).
/// * `svm vmcalls` (required for the instance API).
#[macro_export]
macro_rules! include_svm_wasmer_instance_api {
    ($KV:ident, $PS:ident, $PC: ident) => {
        use std::ffi::c_void;

        /// Injecting the `svm vmcalls` backed by page-cache `$PC` into this file
        include_wasmer_svm_vmcalls!($PC);

        /// Creates a new `wasmer` import object.
        /// The import object will include imports of two flavors:
        /// * external vmcalls (i.e: node vmcalls)
        /// * internal vmcalls (i.e: register/storage/etc vmcalls)
        #[no_mangle]
        pub unsafe extern "C" fn wasmer_svm_import_object(
            import_object_ptr: *mut *mut c_void,
            addr_ptr: *const u8,
            node_data_ptr: *const c_void,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_int,
        ) -> wasmer_runtime_c_api::wasmer_result_t {
            use svm_common::Address;
            use wasmer_runtime::ImportObject;

            // TODO: replace the hardcoded `maximum_pages = 5` and `maximum_slices = 100` with:
            // opts: *const *const wasmer_byte_array,
            // opts_len: libc::c_int,

            let state_gen = lazy_create_svm_state_gen!(
                node_data_ptr,
                Address::from(addr_ptr),
                $KV,
                $PS,
                $PC,
                5,
                100
            );
            let mut import_obj = ImportObject::new_with_data(state_gen);
            append_internal_imports(&mut import_obj);
            append_external_imports(&mut import_obj, imports, imports_len);

            *import_object_ptr = cast_import_obj_to_ptr(import_obj);

            wasmer_result_t::WASMER_OK
        }

        fn append_internal_imports(import_obj: &mut wasmer_runtime::ImportObject) {
            use wasmer_runtime::func;
            use wasmer_runtime_core::import::Namespace;

            let mut ns = Namespace::new();

            ns.insert("mem_to_reg_copy", func!(mem_to_reg_copy));
            // ...
            // ...

            import_obj.register("svm", ns);
        }

        fn append_external_imports(
            import_obj: &mut wasmer_runtime::ImportObject,
            imports: *mut wasmer_import_t,
            imports_len: libc::c_int,
        ) {
            //
        }

        fn cast_import_obj_to_ptr(import_obj: wasmer_runtime::ImportObject) -> *mut c_void {
            let boxed_import_obj = Box::new(import_obj);
            let import_obj_ptr: *mut _ = Box::into_raw(boxed_import_obj);

            import_obj_ptr as *mut c_void
        }
    };
}
