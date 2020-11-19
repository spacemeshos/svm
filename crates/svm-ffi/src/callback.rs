use std::ffi::c_void;

use crate::{svm_byte_array, svm_trap_t};

#[allow(non_camel_case_types)]
pub type svm_func_callback_t = fn(
    env: *const c_void,
    args: *const svm_byte_array,
    results: *mut svm_byte_array,
) -> *mut svm_trap_t;
