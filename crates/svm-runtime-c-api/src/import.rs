use std::ffi::c_void;

use crate::svm_value_type;

/// FFI representation for a byte-array
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_byte_array {
    /// Raw pointer to the beginning of array.
    pub bytes: *const u8,

    /// Array number of bytes,
    pub bytes_len: u32,
}

/// FFI representation for kind of import
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum svm_import_kind {
    #[doc(hidden)]
    SVM_FUNCTION = 0,
}

/// FFI representation for import function signature
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_import_func_sig_t {
    /// Raw pointer to the beginning of function parameter-type array.
    pub params: *const svm_value_type,

    /// Number of parameters
    pub params_len: u32,

    /// Raw pointer to the beginning of function return-type array.
    pub returns: *const svm_value_type,

    /// Number of returns
    pub returns_len: u32,
}

/// FFI representation for import function
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_import_func_t {
    /// Raw pointer to function
    pub func: *const c_void,

    /// Function signature
    pub sig: svm_import_func_sig_t,
}

/// FFI representation for import value
#[allow(non_camel_case_types)]
#[repr(C)]
pub union svm_import_value {
    #[doc(hidden)]
    pub func: *const svm_import_func_t,
}

/// FFI representation for import
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_import_t {
    /// Module name string as `svm_byte_array`
    pub module_name: svm_byte_array,

    /// Import name string as `svm_byte_array`
    pub import_name: svm_byte_array,

    /// Import type (for example: function import)
    pub kind: svm_import_kind,

    /// Import value (for example: pointer to function)
    pub value: svm_import_value,
}
