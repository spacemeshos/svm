use crate::helpers;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

use wasmer_runtime::Ctx as WasmerCtx;

/// Stores 32-bit integer into Register `{reg_bits}:{reg_idx}`.
/// The data is laid out in Big-Endian order.
#[inline]
pub fn reg_set_i32_be(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32, n: u32) {
    let mut buf = [0; 4];
    BigEndian::write_u32(&mut buf, n);

    reg_set(ctx, reg_bits, reg_idx, &buf[..]);
}

/// Stores 32-bit integer into Register `{reg_bits}:{reg_idx}`.
/// The data is laid out in Little-Endian order.
#[inline]
pub fn reg_set_i32_le(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32, n: u32) {
    let mut buf = [0; 4];
    LittleEndian::write_u32(&mut buf, n);

    reg_set(ctx, reg_bits, reg_idx, &buf[..]);
}

/// Stores 64-bit integer into Register `{reg_bits}:{reg_idx}`.
/// The data is laid out in Big-Endian order.
#[inline]
pub fn reg_set_i64_be(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32, n: u64) {
    let mut buf = [0; 8];
    BigEndian::write_u64(&mut buf, n);

    reg_set(ctx, reg_bits, reg_idx, &buf[..]);
}

/// Stores 64-bit integer into Register `{reg_bits}:{reg_idx}`.
/// The data is laid out in Little-Endian order.
#[inline]
pub fn reg_set_i64_le(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32, n: u64) {
    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, n);

    reg_set(ctx, reg_bits, reg_idx, &buf[..]);
}

/// Stores Register `{reg_bits}:{reg_idx}`. see `reg_pop` for restoring.
#[inline]
pub fn reg_push(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.push();
}

/// Restores Register `{reg_bits}:{reg_idx}` into its previous content.
#[inline]
pub fn reg_pop(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.pop();
}

/// Compares two Registers of the same type (`reg_bits` bits each) indexed as `src_idx` and `dst_idx`.
///
/// If both are equals returns `0`,
/// when `src` is smaller returns 1,
/// else (i.e `dst` is smaller) returns -1,
#[inline]
pub fn reg_cmp(ctx: &mut WasmerCtx, reg_bits: u32, src_idx: u32, dst_idx: u32) -> i32 {
    let src = helpers::wasmer_data_reg(ctx.data, reg_bits, src_idx);
    let dst = helpers::wasmer_data_reg(ctx.data, reg_bits, dst_idx);

    let src: &[u8] = src.get();
    let dst: &[u8] = dst.get();

    for (s, d) in src.iter().zip(dst.iter()) {
        if *s > *d {
            return -1;
        } else if *s < *d {
            return 1;
        }
    }

    0
}

#[inline]
fn reg_set(ctx: &mut WasmerCtx, reg_bits: u32, reg_idx: u32, buf: &[u8]) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(buf)
}
