use crate::{svm_byte_array, svm_env_t};

#[allow(non_camel_case_types)]
pub type svm_func_callback_t = unsafe extern "C" fn(
    env: *mut svm_env_t,
    args: *const svm_byte_array,
    results: *mut svm_byte_array,
) -> *mut svm_byte_array;
