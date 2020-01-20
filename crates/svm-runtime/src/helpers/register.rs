use std::ffi::c_void;

use crate::{ctx::SvmCtx, register::Register};

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[inline(always)]
pub fn wasmer_data_reg<'a>(data: *mut c_void, reg_bits: i32, reg_idx: i32) -> &'a mut Register {
    todo!()
}
