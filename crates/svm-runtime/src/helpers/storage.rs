use crate::ctx::SvmCtx;
use crate::helpers;
use std::ffi::c_void;
use svm_storage::ContractStorage;

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut ContractStorage`.
pub fn wasmer_data_storage<'a>(data: *const c_void) -> &'a mut ContractStorage {
    let ctx: &mut SvmCtx = helpers::cast_wasmer_data_to_svm_ctx(data);
    &mut ctx.storage
}
