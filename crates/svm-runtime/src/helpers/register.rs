use std::ffi::c_void;

use crate::{ctx::SvmCtx, register::Register};

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutably borrowed register.
/// Will be used by storage vmcalls.
#[inline]
pub fn wasmer_data_reg<'a>(data: *mut c_void, reg_bits: u32, reg_idx: u32) -> &'a mut Register {
    let ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    ctx.regs.get_reg_mut(reg_bits, reg_idx)
}
