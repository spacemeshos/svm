use std::ffi::c_void;

use crate::{buffer::Buffer, ctx::SvmCtx, helpers};

pub fn wasmer_data_buffer<'a>(data: *mut c_void, buf_id: i32) -> Option<&'a mut Buffer> {
    let ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    ctx.buffers.get_mut(&buf_id)
}

pub fn buffer_create(data: *mut c_void, buf_id: i32, capacity: i32) {
    let mut svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    if svm_ctx.buffers.contains_key(&buf_id) {
        panic!(
            "`buffer_create` failed: Buffer `{}` already exists!",
            buf_id
        );
    }

    svm_ctx.buffers.insert(buf_id, Buffer::new(capacity));
}

pub fn buffer_kill(data: *mut c_void, buf_id: i32) {
    let mut svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    if svm_ctx.buffers.contains_key(&buf_id) == false {
        panic!(
            "`buffer_create` failed: Buffer `{}` doesn't exists!",
            buf_id
        );
    }

    svm_ctx.buffers.remove(&buf_id);
}

pub fn buffer_copy_to_storage(
    data: *mut c_void,
    buf_id: i32,
    buf_offset: i32,
    page_idx: i32,
    page_offset: i32,
    len: i32,
) {
    let buffer =
        wasmer_data_buffer(data, buf_id).expect(&format!("Buffer `{}` doesn't exist!", buf_id));

    let storage = helpers::wasmer_data_app_storage(data);
    let layout = helpers::page_slice_layout(page_idx, page_offset, len);

    let data = buffer.read(buf_offset, len);
    storage.write_page_slice(&layout, data);
}
