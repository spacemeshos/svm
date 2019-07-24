use std::ffi::c_void;

use svm_common::Address;
use svm_wasmer::ctx::SvmCtx;
use svm_wasmer::*;

use svm_storage::memory::{MemKVStore, MemPageCache32, MemPages};

use wasmer_runtime::{func, imports, ImportObject};
use wasmer_runtime_c_api::{
    import::wasmer_import_t, instance::wasmer_instance_t, wasmer_byte_array,
};

include_wasmer_svm_vmcalls!(MemPageCache32);

#[repr(C)]
#[derive(Clone)]
struct wasmer_svm_import_object_t {
    addr: *const u8,
    imports: *mut wasmer_import_t,
    imports_len: libc::c_int,
}

#[no_mangle]
pub unsafe extern "C" fn wasmer_svm_import_object(
    addr_ptr: *const u8,
    node_data: *const c_void,
    imports: *mut wasmer_import_t,
    imports_len: libc::c_int,
) -> *const c_void {
    let addr: Address = Address::from(addr_ptr);

    // TODO: replace the hardcoded `maximum_pages = 5` and `maximum_slices = 100` with:
    // opts: *const *const wasmer_byte_array,
    // opts_len: libc::c_int,

    let import_object = imports! {
        lazy_create_svm_state_gen!(node_data, addr, MemKVStore, MemPages, MemPageCache32, 5, 100),

        "svm" => {
            "mem_to_reg_copy" => func!(mem_to_reg_copy),
        },
    };

    let boxed_importobj = Box::new(import_object);
    let importobj: *const _ = Box::into_raw(boxed_importobj) as *const _;

    importobj as *const c_void
}

// #[no_mangle]
// pub unsafe extern "C" fn wasmer_svm_instantiate(
//     instance: *mut *mut wasmer_instance_t,
//     wasm_bytes: *mut u8,
//     wasm_bytes_len: u32,
// ) {
// }
