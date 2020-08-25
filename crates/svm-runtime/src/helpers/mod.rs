use std::ffi::c_void;

use crate::ctx::SvmCtx;

mod data_wrapper;
mod gas;
mod host_ctx;
mod logs;
mod storage;

pub use data_wrapper::DataWrapper;
pub use gas::{wasmer_gas_used, wasmer_use_gas};
pub use logs::wasmer_data_logs;
pub use storage::wasmer_data_app_storage;

#[inline]
pub fn wasmer_data_svm<'a>(data: *mut c_void) -> &'a mut SvmCtx {
    unsafe { svm_common::from_raw_mut::<SvmCtx>(data) }
}
