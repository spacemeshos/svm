use std::io;
use std::string::FromUtf8Error;

use svm_ffi::svm_byte_array;
use svm_runtime::error::ValidateError;
use svm_types::Type;

pub(crate) unsafe fn raw_validate_error(err: &ValidateError, raw_err: *mut svm_byte_array) {
    let s = format!("{}", err);
    raw_error(s, raw_err);
}

pub(crate) unsafe fn raw_io_error(err: io::Error, raw_err: *mut svm_byte_array) {
    let s = format!("{}", err);
    raw_error(s, raw_err);
}

pub(crate) unsafe fn raw_utf8_error<T>(
    utf8_res: Result<T, FromUtf8Error>,
    raw_err: *mut svm_byte_array,
) {
    let utf8_err = utf8_res.err().unwrap();

    raw_error(utf8_err.to_string(), raw_err);
}

pub(crate) unsafe fn raw_error(s: String, raw_err: *mut svm_byte_array) {
    let ty = Type::Str("runtime-c-api-error");
    let err: svm_byte_array = (ty, s).into();

    let raw_err = &mut *raw_err;

    *raw_err = err;
}
