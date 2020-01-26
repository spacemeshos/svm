use crate::{buffer::Buffer, ctx::SvmCtx, helpers};

use wasmer_runtime::Ctx as WasmerCtx;

pub fn buffer_create(ctx: &mut WasmerCtx, buf_id: u32, capacity: u32) {
    helpers::buffer_create(ctx.data, buf_id, capacity)
}

pub fn buffer_kill(ctx: &mut WasmerCtx, buf_id: u32) {
    helpers::buffer_kill(ctx.data, buf_id);
}

pub fn buffer_freeze(ctx: &mut WasmerCtx, buf_id: u32) {
    helpers::buffer_freeze(ctx.data, buf_id);
}

pub fn buffer_copy_to_storage(
    ctx: &mut WasmerCtx,
    buf_id: u32,
    buf_offset: u32,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) {
    helpers::buffer_copy_to_storage(ctx.data, buf_id, buf_offset, page_idx, page_offset, count);
}

pub fn buffer_copy_to_reg(
    ctx: &mut WasmerCtx,
    buf_id: u32,
    buf_offset: u32,
    reg_bits: u32,
    reg_idx: u32,
    count: u32,
) {
    helpers::buffer_copy_to_reg(ctx.data, buf_id, buf_offset, reg_bits, reg_idx, count);
}
