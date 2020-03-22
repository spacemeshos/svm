use crate::helpers;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

use wasmer_runtime::Ctx as WasmerCtx;

/// Reads `Host Context` field indexed `field_idx` into Register `{reg_bits}:{reg_idx}`
pub fn host_ctx_read_into_reg(ctx: &mut WasmerCtx, field_idx: u32, reg_bits: u32, reg_idx: u32) {
    let host_ctx = helpers::wasmer_data_host_ctx(ctx.data);
    let buf = host_ctx.get(field_idx).unwrap();

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(buf);
}

/// Reads `Host Context` field indexed `field_idx` and interprets it as a 32-bit integer in Little-Endian order.
pub fn host_ctx_read_i32_le(ctx: &mut WasmerCtx, field_idx: u32) -> u32 {
    host_ctx_read_int::<LittleEndian>(ctx, field_idx) as u32
}

/// Reads `Host Context` field indexed `field_idx` and interprets it as a 32-bit integer in Big-Endian order.
pub fn host_ctx_read_i32_be(ctx: &mut WasmerCtx, field_idx: u32) -> u32 {
    host_ctx_read_int::<BigEndian>(ctx, field_idx) as u32
}

/// Reads `Host Context` field indexed `field_idx` and interprets it as a 64-bit integer in Little-Endian order.
pub fn host_ctx_read_i64_le(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    host_ctx_read_int::<LittleEndian>(ctx, field_idx)
}

/// Reads `Host Context` field indexed `field_idx` and interprets it as a 64-bit integer in Big-Endian order.
pub fn host_ctx_read_i64_be(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    host_ctx_read_int::<BigEndian>(ctx, field_idx)
}

fn host_ctx_read_int<T: ByteOrder>(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    let host_ctx = helpers::wasmer_data_host_ctx(ctx.data);
    let buf = host_ctx.get(field_idx).unwrap();
    let len = buf.len();

    T::read_uint(buf, len)
}
