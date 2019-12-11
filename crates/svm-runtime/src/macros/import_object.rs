use crate::ctx::SvmCtx;
use crate::ctx_data_wrapper::SvmCtxDataWrapper;
use crate::opts::Opts;

use svm_common::{Address, State};
use svm_storage::traits::PageCache;
use svm_storage::{ContractPages, ContractStorage};

use std::ffi::c_void;

pub fn create_svm_ctx<'a>(
    addr: Address,
    state: State,
    node_data: *const c_void,
    storage_builder: &Box<dyn Fn(Address, State, &Opts) -> ContractStorage>,
    opts: &Opts,
) -> &'a SvmCtx {
    log::trace!("create_svm_ctx...");

    let storage = storage_builder(addr, state, opts);
    let ctx = SvmCtx::new(SvmCtxDataWrapper::new(node_data), storage);
    let boxed_ctx = Box::new(ctx);

    Box::leak(boxed_ctx)
}

#[inline(always)]
pub fn cast_wasmer_data_to_svm_ctx<'a>(data: *const c_void) -> &'a mut SvmCtx {
    unsafe { &mut *(data as *mut SvmCtx) }
}
