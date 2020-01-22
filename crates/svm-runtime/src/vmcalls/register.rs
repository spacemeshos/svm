use crate::helpers;

#[inline]
pub fn reg_push(ctx: &mut wasmer_runtime::Ctx, reg_bits: i32, reg_idx: i32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.push();
}

#[inline]
pub fn reg_pop(ctx: &mut wasmer_runtime::Ctx, reg_bits: i32, reg_idx: i32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.pop();
}
