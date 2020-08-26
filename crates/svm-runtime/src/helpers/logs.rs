use std::ffi::c_void;

use svm_types::receipt::Log;

use crate::Context;

#[inline]
pub fn wasmer_data_logs<'a>(data: *mut c_void) -> Vec<Log> {
    todo!()
    // let mut ctx = unsafe { svm_common::from_raw_mut::<Context>(data) };

    // ctx.take_logs()
}
