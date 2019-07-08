use std::cell::Cell;
use wasmer_runtime_core::{memory::MemoryView, vm::Ctx};

// use super::register::{Register128, Register64};
//
// #[repr(C)]
// pub struct CtxRegs {
//     regs_64: [Register64; 8],
//
//     regs_128: [Register64; 8],
// }

fn mem_to_reg_copy(ctx: &mut Ctx, src_mem_ptr: i32, dst_reg: i32, offset: i32, len: i32) {
    let start = src_mem_ptr as usize;
    let end = start + len as usize;

    let view: MemoryView<u8> = ctx.memory(0).view();
    let bytes: &[Cell<u8>] = &view[start..end];

    // ctx -- ? --> `dst_reg`
}

fn reg_copy_to_mem_copy(_ctx: &mut Ctx, reg: i32, mem_ptr: i32, mem_len: i32) {
    //
}

fn storage_read_to_reg(_ctx: &mut Ctx, page: i32, offset: i32, len: i32, reg: i32) {
    //
}

fn storage_set_from_reg(_ctx: &mut Ctx, page: i32, offset: i32, reg: i32) {
    //
}
