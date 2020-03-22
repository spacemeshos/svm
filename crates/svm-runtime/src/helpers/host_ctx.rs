use std::ffi::c_void;

use crate::ctx::SvmCtx;

use svm_app::types::HostCtx;

/// Given Wasmer Context data, trasmutes it into `SvmCtx` and borrows its inner `host_ctx` field.
#[inline]
pub fn wasmer_data_host_ctx<'a>(data: *mut c_void) -> &'a HostCtx {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };
    unsafe { &*(svm_ctx.host_ctx) }
}
