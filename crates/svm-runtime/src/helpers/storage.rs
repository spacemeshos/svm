use std::ffi::c_void;

use crate::ctx::SvmCtx;

use svm_storage::ContractStorage;

/// Extracts the `wasmer` instance context `data` field (of type `*mut c_void`) into `&mut ContractStorage`.
#[inline(always)]
pub fn wasmer_data_contract_storage<'a>(data: *mut c_void) -> &'a mut ContractStorage {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };
    &mut svm_ctx.storage
}
