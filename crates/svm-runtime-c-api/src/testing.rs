use std::ffi::c_void;

use wasmer_runtime_c_api::instance::wasmer_instance_context_t;
use wasmer_runtime_core::vm::Ctx;

use svm_runtime::ctx::SvmCtx;
use svm_runtime::helpers::cast_ptr_to_svm_ctx;

pub unsafe fn svm_register_get(
    raw_ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
) -> *const u8 {
    let ctx = cast_to_wasmer_ctx(raw_ctx);
    let reg = svm_runtime::helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.as_ptr()
}

pub unsafe fn svm_node_data_get<'a, T>(raw_ctx: *mut wasmer_instance_context_t) -> &'a mut T {
    let ctx = cast_to_wasmer_ctx(raw_ctx);
    let svm_ctx = cast_ptr_to_svm_ctx(ctx.data);

    &mut *(svm_ctx.node_data as *mut T)
}

// pub unsafe fn alloc_ptr() -> *mut *mut c_void {
//     let ptr = std::ptr::null_mut();
//     let ptr: Box<*mut c_void> = Box::new(ptr);
//
//     Box::into_raw(ptr)
// }

pub fn alloc_ptr() -> *mut *mut c_void {
    let ptr_size: usize = std::mem::size_of::<*mut *mut c_void>();
    let layout = std::alloc::Layout::from_size_align(ptr_size, std::mem::align_of::<u8>()).unwrap();

    unsafe { std::alloc::alloc(layout) as _ }
}

unsafe fn cast_to_wasmer_ctx<'a>(ctx: *mut wasmer_instance_context_t) -> &'a mut Ctx {
    &mut *(ctx as *mut Ctx)
}
