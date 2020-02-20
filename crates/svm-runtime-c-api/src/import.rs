use std::{ffi::c_void, ptr::NonNull};

use crate::svm_value_type;

/// Represents an `Import` kind
#[allow(non_camel_case_types)]
pub enum svm_import_kind {
    #[doc(hidden)]
    SVM_FUNCTION,
}

/// FFI representation for import function signature
#[allow(non_camel_case_types)]
pub struct svm_import_func_sig_t {
    /// Function params types
    pub params: Vec<svm_value_type>,

    /// Function returns types
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

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub enum svm_import_value {
    #[doc(hidden)]
    Func(svm_import_func_t),
}

/// An import declaration
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
