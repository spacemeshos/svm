use crate::helpers;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

use wasmer_runtime::Ctx as WasmerCtx;

/// Reads host context field with index `field_idx` into register `{reg_bits}:{reg_idx}`
pub fn host_ctx_read_into_reg(ctx: &mut WasmerCtx, field_idx: u32, reg_bits: u32, reg_idx: u32) {
    let host_ctx = helpers::wasmer_data_host_ctx(ctx.data);
    let buf = host_ctx.get(field_idx).unwrap();

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(buf);
}

pub fn host_ctx_read_i32_le(ctx: &mut WasmerCtx, field_idx: u32) -> u32 {
    host_ctx_read_int::<LittleEndian>(ctx, field_idx) as u32
}

pub fn host_ctx_read_i32_be(ctx: &mut WasmerCtx, field_idx: u32) -> u32 {
    host_ctx_read_int::<BigEndian>(ctx, field_idx) as u32
}

pub fn host_ctx_read_i64_le(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    host_ctx_read_int::<LittleEndian>(ctx, field_idx)
}

pub fn host_ctx_read_i64_be(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    host_ctx_read_int::<BigEndian>(ctx, field_idx)
}

fn host_ctx_read_int<T: ByteOrder>(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    let host_ctx = helpers::wasmer_data_host_ctx(ctx.data);
    let buf = host_ctx.get(field_idx).unwrap();
    let len = buf.len();

    T::read_uint(buf, len)
}
