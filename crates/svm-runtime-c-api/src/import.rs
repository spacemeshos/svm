use std::{
    default::Default,
    ffi::c_void,
    ptr::{self, NonNull},
    string::FromUtf8Error,
};

use crate::svm_value_type;

/// FFI representation for a byte-array
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct svm_byte_array {
    /// Raw pointer to the beginning of array.
    pub bytes: *const u8,

    /// Number of bytes,
    pub length: u32,
}

impl Default for svm_byte_array {
    fn default() -> Self {
        Self {
            bytes: ptr::null(),
            length: 0,
        }
    }
}

impl From<svm_byte_array> for Result<String, FromUtf8Error> {
    fn from(value: svm_byte_array) -> Self {
        let bytes =
            unsafe { std::slice::from_raw_parts(value.bytes as *mut u8, value.length as usize) };

        String::from_utf8(bytes.to_vec())
    }
}

#[allow(non_camel_case_types)]
pub enum svm_import_kind {
    #[doc(hidden)]
    SVM_FUNCTION = 0,
}

/// FFI representation for import function signature
#[allow(non_camel_case_types)]
pub struct svm_import_func_sig_t {
    pub params: Vec<svm_value_type>,

    pub returns: Vec<svm_value_type>,
}

/// FFI representation for import function
#[allow(non_camel_case_types)]
pub struct svm_import_func_t {
    /// Raw pointer to function
    pub func: NonNull<c_void>,

    /// Function signature
    pub sig: svm_import_func_sig_t,
}

#[allow(non_camel_case_types)]
pub enum svm_import_value {
    #[doc(hidden)]
    Func(svm_import_func_t),
}

#[allow(non_camel_case_types)]
pub struct svm_import_t {
    /// Module name string as `svm_byte_array`
    pub module_name: String,

    /// Import name string as `svm_byte_array`
    pub import_name: String,

    /// Import type (for example: function import)
    pub kind: svm_import_kind,

    /// Import value (for example: a pointer to function)
    pub value: svm_import_value,
}
