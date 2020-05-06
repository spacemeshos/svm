use std::ffi::c_void;

use crate::ctx::SvmCtx;

use svm_storage::AppStorage;
use svm_storage2::app::AppStorage as AppStorage2;

/// Extracts the `wasmer` instance context `data` field (of type `*mut c_void`) into `&mut AppStorage`.
#[inline]
pub fn wasmer_data_app_storage<'a>(data: *mut c_void) -> &'a mut AppStorage {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };
    &mut svm_ctx.storage
}

#[inline]
pub fn wasmer_data_app_storage2<'a>(data: *mut c_void) -> &'a mut AppStorage2 {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };
    &mut svm_ctx.storage2
}
