use std::ffi::c_void;
use std::sync::Arc;

use svm_common::Address;
use svm_wasmer::ctx::SvmCtx;
use svm_wasmer::*;

use svm_storage::memory::{MemKVStore, MemPageCache32, MemPages};

use wasmer_runtime::{func, imports, ImportObject};
use wasmer_runtime_c_api::{
    export::wasmer_import_export_value,
    import::{wasmer_import_func_t, wasmer_import_t},
    instance::wasmer_instance_t,
    wasmer_byte_array, wasmer_result_t,
};
use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    import::Namespace,
    types::{FuncSig, Type},
};

/// Injecting the `svm vmcalls` backed by page-cache `MemPageCache32` into this file
include_wasmer_svm_vmcalls!(MemPageCache32);

#[no_mangle]
pub unsafe extern "C" fn wasmer_svm_import_object(
    import_object_ptr: *mut *mut c_void,
    addr_ptr: *const u8,
    node_data_ptr: *const c_void,
    imports: *mut wasmer_import_t,
    imports_len: libc::c_int,
) -> wasmer_result_t {
    // TODO: replace the hardcoded `maximum_pages = 5` and `maximum_slices = 100` with:
    // opts: *const *const wasmer_byte_array,
    // opts_len: libc::c_int,

    let state_gen = lazy_create_svm_state_gen!(
        node_data_ptr,
        Address::from(addr_ptr),
        MemKVStore,
        MemPages,
        MemPageCache32,
        5,
        100
    );
    let mut import_obj = ImportObject::new_with_data(state_gen);
    append_internal_imports(&mut import_obj);
    append_external_imports(&mut import_obj, imports, imports_len);

    *import_object_ptr = cast_import_obj_to_ptr(import_obj);

    wasmer_result_t::WASMER_OK
}

fn append_internal_imports(import_obj: &mut ImportObject) {
    let mut ns = Namespace::new();

    ns.insert("mem_to_reg_copy", func!(mem_to_reg_copy));
    // ...
    // ...

    import_obj.register("svm", ns);
}

fn append_external_imports(
    import_obj: &mut ImportObject,
    imports: *mut wasmer_import_t,
    imports_len: libc::c_int,
) {
    //
}

fn cast_import_obj_to_ptr(import_obj: ImportObject) -> *mut c_void {
    let boxed_import_obj = Box::new(import_obj);
    let import_obj_ptr: *mut _ = Box::into_raw(boxed_import_obj);

    import_obj_ptr as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    struct NodeData {
        ip: [u8; 4],
        os: String,
    }

    #[no_mangle]
    unsafe extern "C" fn get_balance(ctx: &wasmer_runtime::Ctx, addr: i32) -> i64 {
        return 123;
    }

    fn cast_str_to_wasmer_byte_array(s: String) -> wasmer_byte_array {
        let bytes_vec = s.into_bytes();
        let bytes_len: u32 = bytes_vec.len() as u32;

        let boxed_bytes = Box::new(bytes_vec);
        let bytes: *const u8 = Box::into_raw(boxed_bytes) as *const u8;

        wasmer_byte_array { bytes, bytes_len }
    }

    macro_rules! func_as_wasmer_import_export_value {
        ($func: path, $params: expr, $returns: expr) => {
            unsafe {
                let export = Box::new(Export::Function {
                    func: FuncPointer::new($func as _),
                    ctx: Context::Internal,
                    signature: Arc::new(FuncSig::new($params, $returns)),
                });

                Box::into_raw(export) as *mut wasmer_import_func_t
            }
        };
    }

    #[test]
    fn create_import_object() {
        let import_object: *const *const c_void;

        let node_data = NodeData {
            ip: [192, 168, 1, 10],
            os: String::from("mac"),
        };

        let addr_ptr: *const u8 = Address::from(0x11_22_33_44).as_ptr();
        let node_data_ptr: *const c_void = &node_data as *const NodeData as *const _;

        let imports: *mut wasmer_import_t;
        let imports_len: libc::c_int = 1;

        let params = vec![Type::I32];
        let returns = vec![Type::I64];

        let get_balance_ptr: *mut wasmer_import_export_value =
            func_as_wasmer_import_export_value!(get_balance, params, returns) as *mut _;

        // wasmer_svm_import_object(import_object, addr_ptr, node_data_ptr,
    }
}
