use crate::helpers;

use wasmer_runtime::Ctx as WasmerCtx;

/// Creates a new buffer with id `buf_id` and capacity `cap` for running App.
pub fn buffer_create(ctx: &mut WasmerCtx, buf_id: u32, cap: u32) {
    helpers::buffer_create(ctx.data, buf_id, cap)
}

/// Kills buffer `buf_id` for running `App`.
pub fn buffer_kill(ctx: &mut WasmerCtx, buf_id: u32) {
    helpers::buffer_kill(ctx.data, buf_id);
}

/// Turns buffer `buf_id` into read-only for running `App`.
pub fn buffer_freeze(ctx: &mut WasmerCtx, buf_id: u32) {
    helpers::buffer_freeze(ctx.data, buf_id);
}

/// Copies buffer `buf_id` bytes under `buf_offset, buf_offset + 1, ..., buf_offset + count - 1`
/// to running App's storage, page `page_idx` starting at offset `page_offset`.
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

/// Copies buffer `buf_id` bytes under `buf_offset, buf_offset + 1, ..., buf_offset + count - 1`
/// to running App's Register of type `reg_bits` and index `reg_idx`.
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
