use std::ffi::c_void;

use svm_types::receipt::Log;

use crate::Context;

#[inline]
pub fn wasmer_data_logs<'a>(data: *mut c_void) -> Vec<Log> {
    let mut svm_ctx = unsafe { svm_common::from_raw_mut::<Context>(data) };

    svm_ctx.take_logs()
}
