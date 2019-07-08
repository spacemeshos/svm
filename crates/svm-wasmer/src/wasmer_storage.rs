use std::cell::Cell;
use wasmer_runtime_core::{memory::Memory, memory::MemoryView, vm::Ctx};

use super::wasmer_register::WasmerReg64;

pub const REGS_64_COUNT: usize = 8;

#[repr(C)]
pub struct CtxRegs {
    pub(crate) regs_64: [WasmerReg64; REGS_64_COUNT],
}

impl CtxRegs {
    fn new() -> Self {
        Self {
            regs_64: [WasmerReg64::new(); REGS_64_COUNT],
        }
    }
}

fn mem_to_reg_copy(ctx: &mut Ctx, src_mem_ptr: i32, dst_reg: i32, offset: i32, len: i32) {
    assert!(dst_reg >= 0 && dst_reg < REGS_64_COUNT as i32);

    let start = src_mem_ptr as usize;
    let end = start + len as usize;

    let view: MemoryView<u8> = ctx.memory(0).view();
    let cells: &[Cell<u8>] = &view[start..end];

    // let regs: &mut CtxRegs = ctx ??
    let mut regs = CtxRegs::new();

    let reg = &mut regs.regs_64[dst_reg as usize];

    reg.copy_from_wasmer_mem(cells);
}

fn reg_copy_to_mem_copy(ctx: &mut Ctx, src_reg: i32, dst_mem_ptr: i32, len: i32) {
    assert!(src_reg >= 0 && src_reg < REGS_64_COUNT as i32);

    let start = dst_mem_ptr as usize;
    let end = start + len as usize;

    let view: MemoryView<u8> = ctx.memory(0).view();
    let cells: &[Cell<u8>] = &view[start..end];

    // let regs: &mut CtxRegs = ctx ??
    let mut regs = CtxRegs::new();

    let reg = &mut regs.regs_64[src_reg as usize];

    reg.copy_to_wasmer_mem(cells);
}

fn storage_read_to_reg(_ctx: &mut Ctx, page: i32, offset: i32, len: i32, reg: i32) {
    //
}

fn storage_set_from_reg(_ctx: &mut Ctx, page: i32, offset: i32, reg: i32) {
    //
}
