use crate::helpers;

use wasmer_runtime::Ctx as WasmerCtx;

#[inline]
pub fn reg_push(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.push();
}

#[inline]
pub fn reg_pop(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.pop();
}

#[inline]
pub fn reg_eql(ctx: &mut WasmerCtx, reg_bits: u32, src_idx: u32, dst_idx: u32) -> u32 {
    return 0;
}
