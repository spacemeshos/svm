use super::page_slice_layout;
use crate::{buffer::Buffer, ctx::SvmCtx, helpers};

pub fn buffer_create(ctx: &mut wasmer_runtime::Ctx, buf_id: i32, capacity: i32) {
    let mut svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(ctx.data) };

    if svm_ctx.buffers.contains_key(&buf_id) {
        panic!(
            "`buffer_create` failed: Buffer `{}` already exists!",
            buf_id
        );
    }

    svm_ctx.buffers.insert(buf_id, Buffer::new(capacity));
}

pub fn buffer_kill(ctx: &mut wasmer_runtime::Ctx, buf_id: i32) {
    let buffer = helpers::wasmer_data_buffer(ctx.data, buf_id);
    buffer.clear();
}

pub fn buffer_copy_to_reg(
    ctx: &mut wasmer_runtime::Ctx,
    buf_id: i32,
    buf_offset: i32,
    reg_bits: i32,
    reg_idx: i32,
    len: i32,
) {
    let buffer = helpers::wasmer_data_buffer(ctx.data, buf_id);
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);

    let data = buffer.read(buf_offset, len);
    reg.set(data);
}

pub fn buffer_copy_to_storage(
    ctx: &mut wasmer_runtime::Ctx,
    buf_id: i32,
    buf_offset: i32,
    page_idx: i32,
    page_offset: i32,
    len: i32,
) {
    let buffer = helpers::wasmer_data_buffer(ctx.data, buf_id);
    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let layout = page_slice_layout(page_idx, page_offset, len);

    let data = buffer.read(buf_offset, len);
    storage.write_page_slice(&layout, data);
}
