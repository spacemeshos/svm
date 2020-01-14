use std::ffi::c_void;

use crate::{buffer::Buffer, ctx::SvmCtx};

pub fn wasmer_data_buffer<'a>(data: *mut c_void, buf_id: i32) -> &'a mut Buffer {
    let ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    ctx.buffers
        .get_mut(&buf_id)
        .expect(&format!("Buffer `{}` doesn't exist!", buf_id))
}
