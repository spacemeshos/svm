use crate::ctx::SvmCtx;
use crate::ctx_data_wrapper::SvmCtxDataWrapper;
use crate::opts::Opts;

use svm_common::{Address, State};
use svm_storage::traits::PageCache;
use svm_storage::{ContractPages, ContractStorage};

use std::ffi::c_void;

#[inline(always)]
pub fn cast_wasmer_data_to_svm_ctx<'a>(data: *const c_void) -> &'a mut SvmCtx {
    unsafe { &mut *(data as *mut SvmCtx) }
}
