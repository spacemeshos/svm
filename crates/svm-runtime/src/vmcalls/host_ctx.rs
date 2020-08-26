use crate::{helpers, use_gas, Context};

use byteorder::{ByteOrder, LittleEndian};

/// Returns the `Host Context` field `field_idx` as i64 (Little-Endian)
pub fn host_get64(ctx: &mut Context, field_idx: u32) -> u64 {
    use_gas!("host_get64", ctx);

    host_ctx_read_int::<LittleEndian>(ctx, field_idx)
}

fn host_ctx_read_int<T: ByteOrder>(ctx: &Context, field_idx: u32) -> u64 {
    let host_ctx = &ctx.borrow().host_ctx;

    let buf = host_ctx.get(field_idx).unwrap();
    let len = buf.len();

    T::read_uint(buf, len)
}
