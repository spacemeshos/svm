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
