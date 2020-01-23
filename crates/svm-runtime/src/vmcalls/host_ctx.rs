use crate::{ctx::SvmCtx, helpers};

use wasmer_runtime::Ctx as WasmerCtx;

/// Reads host context field with index `field_idx` into register `{reg_bits}:{reg_idx}`
pub fn host_ctx_read_into_reg(ctx: &mut WasmerCtx, field_idx: i32, reg_bits: i32, reg_idx: i32) {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(ctx.data) };
    let host_ctx = unsafe { &*(svm_ctx.host_ctx) };

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let slice = host_ctx.get(field_idx).unwrap();

    reg.set(slice);
}

pub fn host_ctx_read_i8(ctx: &mut WasmerCtx, field_idx: i32) -> i32 {
    todo!()
}

pub fn host_ctx_read_i16_le(ctx: &mut WasmerCtx, field_idx: i32) -> i32 {
    todo!()
}

pub fn host_ctx_read_i16_be(ctx: &mut WasmerCtx, field_idx: i32) -> i32 {
    todo!()
}

pub fn host_ctx_read_i32_le(ctx: &mut WasmerCtx, field_idx: i32) -> i32 {
    todo!()
}

pub fn host_ctx_read_i32_be(ctx: &mut WasmerCtx, field_idx: i32) -> i32 {
    todo!()
}

pub fn host_ctx_read_i64_le(ctx: &mut WasmerCtx, field_idx: i32) -> i64 {
    todo!()
}

pub fn host_ctx_read_i64_be(ctx: &mut WasmerCtx, field_idx: i32) -> i64 {
    todo!()
}
