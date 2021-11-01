use std::convert::{Infallible, TryInto};
use std::ops::FromResidual;

/// FFI representation for function result type.
///
/// [`svm_result_t`] effectively has three variants:
///
/// - Error variant.
/// - Receipt variant.
/// - No data, just okay state.
///
/// Please note that [`svm_result_t`] implements [`std::ops::Try`], so you can
/// effectively use `?` everywhere and it will automatically return an
/// [`svm_result_t::new_error()`] if necessary.
///
/// # Memory management
///
/// All [`svm_result_t`] instances allocate memory using the system allocator,
/// so it's very easy to free contents from C and other languages.
///
/// ```c, no_run
/// free(result->receipt);
/// free(result->error);
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct svm_result_t {
    receipt: *mut u8,
    error: *mut u8,
    buf_size: u32,
}

impl svm_result_t {
    /// A successful [`svm_result_t`], with neither a receipt nor error information.
    pub const OK: Self = Self {
        receipt: std::ptr::null_mut(),
        error: std::ptr::null_mut(),
        buf_size: 0,
    };

    /// Creates a new [`svm_result_t`] which contains an error.
    pub fn new_error(data: &[u8]) -> Self {
        let mut new_data = Vec::with_capacity(data.len());
        new_data.extend_from_slice(data);

        Self {
            receipt: std::ptr::null_mut(),
            error: new_data.leak().as_mut_ptr(),
            buf_size: data.len().try_into().unwrap(),
        }
    }

    /// Creates a new [`svm_result_t`] which contains a receipt.
    pub fn new_receipt(data: &[u8]) -> Self {
        let mut new_data = Vec::with_capacity(data.len());
        new_data.extend_from_slice(data);

        Self {
            receipt: new_data.leak().as_mut_ptr(),
            error: std::ptr::null_mut(),
            buf_size: data.len().try_into().unwrap(),
        }
    }

    /// Returns [`Some(bytes)`] if and only if `self` is a transaction receipt.
    pub fn receipt(&self) -> Option<&[u8]> {
        if !self.receipt.is_null() {
            unsafe {
                Some(std::slice::from_raw_parts(
                    self.receipt,
                    self.buf_size as usize,
                ))
            }
        } else {
            None
        }
    }

    /// Returns whether `self` is either a receipt or simply equal to
    /// [`svm_result_t::OK`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use svm_runtime_ffi::svm_result_t;
    ///
    /// assert!(svm_result_t::OK.is_ok());
    /// ```
    pub fn is_ok(&self) -> bool {
        self.error.is_null()
    }

    /// Returns whether `self` is not equal to [`svm_result_t::OK`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use svm_runtime_ffi::svm_result_t;
    ///
    /// assert!(!svm_result_t::OK.is_err());
    /// assert!(svm_result_t::new_error(b"err foobar").is_err());
    /// ```
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

impl std::ops::Drop for svm_result_t {
    fn drop(&mut self) {
        let len = self.buf_size as usize;
        unsafe {
            let _error = Vec::from_raw_parts(self.error, len, len);
            let _receipt = Vec::from_raw_parts(self.receipt, len, len);
        }
    }
}

impl<E> FromResidual<Result<Infallible, E>> for svm_result_t
where
    E: std::error::Error,
{
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        svm_result_t::new_error(residual.unwrap_err().to_string().as_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn receipt() {
        assert_eq!(
            svm_result_t::new_receipt(b"foobar").receipt().unwrap(),
            b"foobar"
        );
    }
}
