use std::string::FromUtf8Error;

use crate::svm_byte_array;
use svm_app::error::ParseError;

pub(crate) unsafe fn raw_parse_error(parse_err: &ParseError, error: *mut svm_byte_array) {
    let s = format!("{}", parse_err);
    raw_error(s, error);
}

pub(crate) unsafe fn raw_utf8_error<T>(
    utf8_res: Result<T, FromUtf8Error>,
    error: *mut svm_byte_array,
) {
    let utf8_err = utf8_res.err().unwrap();

    raw_error(utf8_err.to_string(), error);
}

pub(crate) unsafe fn raw_error(s: String, error: *mut svm_byte_array) {
    let s = Box::leak(Box::new(s));

    let error = &mut *error;

    error.bytes = s.as_ptr();
    error.length = s.as_bytes().len() as u32;
}
