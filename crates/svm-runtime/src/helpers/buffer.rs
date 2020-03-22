use std::ffi::c_void;

use crate::{
    buffer::{BufferMut, BufferRef},
    ctx::SvmCtx,
    helpers,
};

/// Given Wasmer context data, extracts a borrowed buffer having id `buf_id`
pub fn wasmer_data_buffer<'a>(data: *mut c_void, buf_id: u32) -> Option<&'a mut BufferRef> {
    let ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    ctx.buffers.get_mut(&buf_id)
}

/// Given Wasmer context data, creates a new read/write buffer with id `buf_id` and capacity `capacity`.
/// Buffer is added to Wasmer context data.
pub fn buffer_create(data: *mut c_void, buf_id: u32, capacity: u32) {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    if svm_ctx.buffers.contains_key(&buf_id) {
        panic!(
            "`buffer_create` failed: Buffer `{}` already exists!",
            buf_id
        );
    }

    let buf = BufferMut::new(capacity);
    let buf_ref = BufferRef::Mutable(buf_id, buf);

    svm_ctx.buffers.insert(buf_id, buf_ref);
}

/// Kills buffer with id `buf_id`.
pub fn buffer_kill(data: *mut c_void, buf_id: u32) {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    if svm_ctx.buffers.contains_key(&buf_id) == false {
        panic!("`buffer_kill` failed: Buffer `{}` doesn't exists!", buf_id);
    }

    svm_ctx.buffers.remove(&buf_id);
}

/// Turn buffer with id `buf_id` into a read-only buffer.
pub fn buffer_freeze(data: *mut c_void, buf_id: u32) {
    let svm_ctx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    let entry = svm_ctx.buffers.remove_entry(&buf_id);

    if entry.is_none() {
        panic!(
            "`buffer_freeze` failed: Buffer `{}` doesn't exists!",
            buf_id
        );
    }

    let (.., buf) = entry.unwrap();

    match buf {
        BufferRef::Mutable(.., buf) => {
            let buf = buf.freeze();
            let buf_ref = BufferRef::ReadOnly(buf_id, buf);

            svm_ctx.buffers.insert(buf_id, buf_ref);
        }
        BufferRef::ReadOnly(..) => {
            // do nothing, buffer is already frozen
        }
    }
}

/// Copies buffer `buf_id` bytes `buf_offset, buf_offset + 1, ..., buf_offset + len - 1` into
/// App storage under page `page_idx` starting from offset `page_offset`.
pub fn buffer_copy_to_storage(
    data: *mut c_void,
    buf_id: u32,
    buf_offset: u32,
    page_idx: u32,
    page_offset: u32,
    len: u32,
) {
    let buffer = wasmer_data_buffer(data, buf_id)
        .unwrap_or_else(|| panic!("Buffer `{}` doesn't exist!", buf_id));

    let storage = helpers::wasmer_data_app_storage(data);
    let layout = helpers::page_slice_layout(page_idx, page_offset, len);

    let data = buffer.read(buf_offset, len);
    storage.write_page_slice(&layout, data);
}

/// Copies buffer `buf_id` bytes `buf_offset, buf_offset + 1, ..., buf_offset + len - 1` into
/// App's Register of type `reg_bits` and index `reg_idx`.
pub fn buffer_copy_to_reg(
    data: *mut c_void,
    buf_id: u32,
    buf_offset: u32,
    reg_bits: u32,
    reg_idx: u32,
    len: u32,
) {
    assert!(len * 8 <= reg_bits);

    let buffer = wasmer_data_buffer(data, buf_id)
        .unwrap_or_else(|| panic!("Buffer `{}` doesn't exist!", buf_id));

    let slice = buffer.read(buf_offset, len);
    let reg = helpers::wasmer_data_reg(data, reg_bits, reg_idx);

    reg.set(slice);
}
