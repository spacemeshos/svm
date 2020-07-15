use std::ffi::c_void;

use crate::ctx::SvmCtx;

#[inline]
pub fn wasmer_data_logs<'a>(data: *mut c_void) -> &'a [(Vec<u8>, u32)] {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };
    &mut svm_ctx.logs
}
