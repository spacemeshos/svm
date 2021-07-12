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

///
/// # Examples
///
/// ```rust
/// use svm_runtime_ffi::svm_result_t;
///
/// let truthy = svm_result_t::SVM_SUCCESS;
/// let falsey = svm_result_t::SVM_FAILURE;
///
/// assert_eq!(true, bool::from(truthy));
/// assert_eq!(false, bool::from(falsey));
/// ```
///
impl From<svm_result_t> for bool {
    #[inline]
    fn from(value: svm_result_t) -> bool {
        match value {
            svm_result_t::SVM_SUCCESS => true,
            svm_result_t::SVM_FAILURE => false,
        }
    }
}

impl svm_result_t {
    /// Returns whether equals to `svm_result::SVM_SUCCESS`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use svm_runtime_ffi::svm_result_t;
    ///
    /// let truthy = svm_result_t::SVM_SUCCESS;
    /// let falsey = svm_result_t::SVM_FAILURE;
    ///
    /// assert!(truthy.is_ok());
    /// assert!(!falsey.is_ok());
    /// ```
    ///
    #[inline]
    pub fn is_ok(self) -> bool {
        self.as_bool() == true
    }

    /// Returns whether equals to `svm_result::SVM_FAILURE`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use svm_runtime_ffi::svm_result_t;
    ///
    /// let truthy = svm_result_t::SVM_SUCCESS;
    /// let falsey = svm_result_t::SVM_FAILURE;
    ///
    /// assert!(!truthy.is_err());
    /// assert!(falsey.is_err());
    /// ```
    #[inline]
    pub fn is_err(self) -> bool {
        self.as_bool() == false
    }

    /// Convert to a boolean
    #[inline]
    fn as_bool(self) -> bool {
        self.into()
    }
}
