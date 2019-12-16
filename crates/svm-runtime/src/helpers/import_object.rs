use std::ffi::c_void;

use crate::ctx::SvmCtx;

#[inline(always)]
pub fn cast_wasmer_data_to_svm_ctx<'a>(data: *const c_void) -> &'a mut SvmCtx {
    unsafe { &mut *(data as *mut SvmCtx) }
}
