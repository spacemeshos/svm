use crate::ctx::SvmCtx;
use std::ffi::c_void;

/// Casts a raw `*const void` pointer to `SvmCtx`.
/// It's useful since `wasmer` instances have a field named `data`
/// for saving an arbitrary data requried by the runtime vmcalls.
/// In our case we leverage this `data` field for storing `SvmCtx`.
#[inline(always)]
pub fn cast_ptr_to_svm_ctx<'a>(data: *const c_void) -> &'a mut SvmCtx {
    unsafe { &mut *(data as *mut SvmCtx) }
}
