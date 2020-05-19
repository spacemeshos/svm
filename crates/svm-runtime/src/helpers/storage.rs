use std::ffi::c_void;

use crate::ctx::SvmCtx;

use svm_storage2::app::AppStorage as AppStorage2;

/// Extracts a mutable-borrowed `AppStorage` from `Wasmer` instance's `data`.
#[inline]
pub fn wasmer_data_app_storage2<'a>(data: *mut c_void) -> &'a mut AppStorage2 {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };
    &mut svm_ctx.storage2
}
