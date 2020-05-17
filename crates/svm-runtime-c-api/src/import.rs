use std::{ffi::c_void, ptr::NonNull};

use svm_app::types::WasmType;

use crate::svm_byte_array;

/// Represents an `Import` kind
pub enum ImportKind {
    #[doc(hidden)]
    Function,
}

/// FFI representation for import function signature
pub struct ImportFuncSig {
    /// Function params types
    pub params: Vec<WasmType>,

    /// Function returns types
    pub returns: Vec<WasmType>,
}

/// FFI representation for import function
pub struct ImportFunc {
    /// Raw pointer to function
    pub func: NonNull<c_void>,

    /// Function signature
    pub sig: ImportFuncSig,
}

#[doc(hidden)]
pub enum ImportValue {
    #[doc(hidden)]
    Func(ImportFunc),
}

/// An import declaration
#[allow(non_camel_case_types)]
pub struct Import {
    /// Module name string as `svm_byte_array`
    pub module_name: String,

    /// Import name string as `svm_byte_array`
    pub import_name: String,

    /// Import type (for example: function import)
    pub kind: ImportKind,

    /// Import value (for example: a pointer to function)
    pub value: ImportValue,
}
