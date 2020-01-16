use crate::{buffer::Buffer, ctx::SvmCtx, helpers};

pub fn buffer_create(ctx: &mut wasmer_runtime::Ctx, buf_id: i32, capacity: i32) {
    helpers::buffer_create(ctx.data, buf_id, capacity)
}

pub fn buffer_kill(ctx: &mut wasmer_runtime::Ctx, buf_id: i32) {
    helpers::buffer_kill(ctx.data, buf_id);
}

pub fn buffer_freeze(ctx: &mut wasmer_runtime::Ctx, buf_id: i32) {
    helpers::buffer_freeze(ctx.data, buf_id);
}

pub fn buffer_copy_to_storage(
    ctx: &mut wasmer_runtime::Ctx,
    buf_id: i32,
    buf_offset: i32,
    page_idx: i32,
    page_offset: i32,
    len: i32,
) {
    helpers::buffer_copy_to_storage(ctx.data, buf_id, buf_offset, page_idx, page_offset, len);
}
