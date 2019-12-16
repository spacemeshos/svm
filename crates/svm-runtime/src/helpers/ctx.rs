use crate::ctx::SvmCtx;
use std::ffi::c_void;

#[inline(always)]
pub fn cast_ptr_to_svm_ctx<'a>(data: *const c_void) -> &'a mut SvmCtx {
    unsafe { &mut *(data as *mut SvmCtx) }
}
