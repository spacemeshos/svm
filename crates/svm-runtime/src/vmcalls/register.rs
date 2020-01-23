use crate::helpers;

use wasmer_runtime::Ctx as WasmerCtx;

#[inline]
pub fn reg_push(ctx: &mut WasmerCtx, reg_bits: i32, reg_idx: i32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.push();
}

#[inline]
pub fn reg_pop(ctx: &mut WasmerCtx, reg_bits: i32, reg_idx: i32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.pop();
}
