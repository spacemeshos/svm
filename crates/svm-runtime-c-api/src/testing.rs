use std::ffi::c_void;

use svm_ffi::svm_byte_array;

use svm_runtime::Context;

// /// Allocates `count` imports array, returns a pointer to the first import.
pub fn imports_alloc(count: u32) -> *mut c_void {
    let mut imports = std::ptr::null_mut();

    let res = unsafe { crate::svm_imports_alloc(&mut imports, count) };
    assert!(res.is_ok());

    imports
}
