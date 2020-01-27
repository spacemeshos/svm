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

impl Drop for svm_byte_array {
    fn drop(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(
                self.bytes as *mut u8,
                self.bytes_len as usize,
                self.bytes_len as usize,
            );
        }
    }
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

unsafe fn svm_value_type_array_free(data: *const svm_value_type, len: u32) {
    // no need to iterate over `data` and free each item
    // since `svm_value_type` has no pointers to other heap locations.

    let _ = Vec::from_raw_parts(data as *mut svm_value_type, len as usize, len as usize);
}

impl Drop for svm_import_func_sig_t {
    fn drop(&mut self) {
        unsafe {
            svm_value_type_array_free(self.params, self.params_len);
            svm_value_type_array_free(self.returns, self.returns_len);
        }
    }
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

unsafe fn svm_import_value_free(value: &svm_import_value) {
    let _ = Box::from_raw(value.func as *mut svm_import_func_t);
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

impl Drop for svm_import_t {
    fn drop(&mut self) {
        unsafe {
            svm_import_value_free(&self.value);
        }
    }
}
