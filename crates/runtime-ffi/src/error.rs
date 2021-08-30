use svm_types::Type;

use crate::svm_byte_array;

#[cfg(feature = "default-rocksdb")]
pub(crate) unsafe fn raw_utf8_error<T>(
    utf8_res: Result<T, std::string::FromUtf8Error>,
    raw_err: *mut svm_byte_array,
) {
    let utf8_err = utf8_res.err().unwrap();

    raw_error(utf8_err.to_string(), raw_err);
}

pub(crate) fn raw_error<T>(e: T, raw_err: &mut svm_byte_array)
where
    T: ToString,
{
    let ty = Type::Str("runtime-ffi-error");
    let err: svm_byte_array = (ty, e.to_string()).into();

    *raw_err = err;
}
