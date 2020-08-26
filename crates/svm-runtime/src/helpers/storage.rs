use std::ffi::c_void;

use crate::Context;

use svm_storage::app::AppStorage;

/// Extracts a mutable-borrowed `AppStorage` from `Wasmer` instance's `data`.
#[inline]
pub fn wasmer_data_app_storage<'a>(data: *mut c_void) -> &'a mut AppStorage {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<Context>(data) };
    svm_ctx.storage_mut()
}
