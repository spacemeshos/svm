/// FFI representation for function result type
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum svm_result_t {
    #[doc(hidden)]
    SVM_SUCCESS = 0,

    #[doc(hidden)]
    SVM_FAILURE = 1,
}

impl Into<bool> for svm_result_t {
    #[inline]
    fn into(self) -> bool {
        match self {
            svm_result_t::SVM_SUCCESS => true,
            svm_result_t::SVM_FAILURE => false,
        }
    }
}

impl svm_result_t {
    /// Returns whether equals to `svm_result::SVM_SUCCESS`
    #[inline]
    pub fn is_ok(self) -> bool {
        self.as_bool() == true
    }

    /// Returns whether equals to `svm_result::SVM_FAILURE`
    #[inline]
    pub fn is_err(self) -> bool {
        self.as_bool() == false
    }

    /// Convert to boolean
    #[inline]
    pub fn as_bool(self) -> bool {
        self.into()
    }
}
