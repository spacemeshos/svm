use std::string::FromUtf8Error;

use crate::svm_byte_array;
use svm_runtime::error::ValidateError;

pub(crate) unsafe fn raw_validate_error(err: &ValidateError, raw_err: *mut svm_byte_array) {
    let s = format!("{}", err);
    raw_error(s, raw_err);
}

pub(crate) unsafe fn raw_utf8_error<T>(
    utf8_res: Result<T, FromUtf8Error>,
    error: *mut svm_byte_array,
) {
    let utf8_err = utf8_res.err().unwrap();

    raw_error(utf8_err.to_string(), error);
}

pub(crate) unsafe fn raw_error(s: String, raw_err: *mut svm_byte_array) {
    let s = Box::leak(Box::new(s));

    let raw_err = &mut *raw_err;

    raw_err.bytes = s.as_ptr();
    raw_err.length = s.as_bytes().len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_raw_error() {
        let err_msg = "unexpected error.";

        let mut raw_err = svm_byte_array::default();

        unsafe {
            raw_error(err_msg.to_string(), &mut raw_err);
        }

        let err = String::try_from(raw_err).unwrap();

        assert_eq!(err, err_msg.to_string());
    }
}
