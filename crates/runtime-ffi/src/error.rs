use std::io;

use svm_ffi::svm_byte_array;
use svm_runtime::ValidateError;
use svm_types::Type;

pub(crate) fn raw_validate_error(err: &ValidateError, raw_err: &mut svm_byte_array) {
    let s = format!("{}", err);
    raw_error(s, raw_err);
}

pub(crate) fn raw_io_error(err: io::Error, raw_err: &mut svm_byte_array) {
    let s = format!("{}", err);
    raw_error(s, raw_err);
}

#[cfg(feature = "default-rocksdb")]
pub(crate) unsafe fn raw_utf8_error<T>(
    utf8_res: Result<T, std::string::FromUtf8Error>,
    raw_err: *mut svm_byte_array,
) {
    let utf8_err = utf8_res.err().unwrap();

    raw_error(utf8_err.to_string(), raw_err);
}

pub(crate) fn raw_error(s: String, raw_err: &mut svm_byte_array) {
    let ty = Type::Str("runtime-ffi-error");
    let err: svm_byte_array = (ty, s).into();

    *raw_err = err;
}
