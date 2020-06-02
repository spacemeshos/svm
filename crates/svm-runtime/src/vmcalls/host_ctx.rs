use crate::{helpers, use_gas};

use byteorder::{ByteOrder, LittleEndian};

use wasmer_runtime::Ctx as WasmerCtx;

/// Returns the `Host Context` field `field_idx` as i64 (Little-Endian)
pub fn host_get64(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    use_gas!("host_get64", ctx);

    host_ctx_read_int::<LittleEndian>(ctx, field_idx)
}

fn host_ctx_read_int<T: ByteOrder>(ctx: &mut WasmerCtx, field_idx: u32) -> u64 {
    let host_ctx = helpers::wasmer_data_host_ctx(ctx.data);
    let buf = host_ctx.get(field_idx).unwrap();
    let len = buf.len();

    T::read_uint(buf, len)
}
