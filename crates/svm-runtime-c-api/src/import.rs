use std::ffi::c_void;

use crate::svm_value_type;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_byte_array {
    pub bytes: *const u8,
    pub bytes_len: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum svm_import_kind {
    SVM_FUNCTION = 0,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_import_func_sig_t {
    pub params: *const svm_value_type,
    pub params_len: u32,
    pub returns: *const svm_value_type,
    pub returns_len: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_import_func_t {
    pub func: *const c_void,
    pub sig: svm_import_func_sig_t,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub union svm_import_value {
    pub func: *const svm_import_func_t,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_import_t {
    pub module_name: svm_byte_array,
    pub import_name: svm_byte_array,
    pub kind: svm_import_kind,
    pub value: svm_import_value,
}
