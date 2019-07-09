use std::cell::Cell;
use std::ffi::c_void;
use wasmer_runtime_core::{memory::Memory, memory::MemoryView, vm::Ctx};

use super::wasmer_register::WasmerReg64;

pub const REGS_64_COUNT: usize = 8;

#[repr(C)]
#[derive(Debug)]
pub struct CtxRegs {
    pub(crate) regs_64: [WasmerReg64; REGS_64_COUNT],
}

impl CtxRegs {
    fn new() -> Self {
        let regs_64 = [WasmerReg64::new(); REGS_64_COUNT];

        Self { regs_64 }
    }
}

macro_rules! ctx_regs_reg {
    ($regs: expr, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && $reg_idx < REGS_64_COUNT as i32);

        &mut $regs.regs_64[$reg_idx as usize]
    }};
}

macro_rules! wasmer_ctx_data_regs {
    ($data: expr) => {{
        let data_ptr: *mut CtxRegs = unsafe { $data as *mut _ };

        let data: &mut CtxRegs = unsafe { &mut *data_ptr };

        data
    }};
}

macro_rules! wasmer_ctx_regs {
    ($ctx: expr) => {{
        wasmer_ctx_data_regs!($ctx.data)
    }};
}

macro_rules! wasmer_mem_cells {
    ($ctx: expr, $mem_start: expr, $len: expr) => {{
        let start = $mem_start as usize;
        let end = start + $len as usize;

        &$ctx.memory(0).view()[start..end]
    }};
}

macro_rules! wasmer_reg {
    ($ctx: expr, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && $reg_idx < REGS_64_COUNT as i32);

        let regs = wasmer_ctx_regs!($ctx);

        ctx_regs_reg!(regs, $reg_idx)
    }};
}

fn mem_to_reg_copy(ctx: &mut Ctx, src_mem_ptr: i32, dst_reg: i32, offset: i32, len: i32) {
    let reg = wasmer_reg!(ctx, dst_reg);
    let cells = wasmer_mem_cells!(ctx, src_mem_ptr, len);

    reg.copy_from_wasmer_mem(cells);
}

fn reg_copy_to_mem_copy(ctx: &mut Ctx, src_reg: i32, dst_mem_ptr: i32, len: i32) {
    let reg = wasmer_reg!(ctx, src_reg);
    let cells = wasmer_mem_cells!(ctx, dst_mem_ptr, len);

    reg.copy_to_wasmer_mem(cells);
}

fn storage_read_to_reg(_ctx: &mut Ctx, page: i32, offset: i32, len: i32, reg: i32) {
    //
}

fn storage_set_from_reg(_ctx: &mut Ctx, page: i32, offset: i32, reg: i32) {
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::c_void;
    use wasmer_runtime_core::{import::ImportObject, vm::Ctx};

    fn allocate_ctx_regs() -> *mut c_void {
        let mut ctx_regs = CtxRegs::new();

        unsafe { &mut ctx_regs as *mut _ as *mut c_void }
    }

    fn dtor(data: *mut c_void) {}

    fn state_creator() -> (*mut c_void, fn(*mut c_void)) {
        let data: *mut c_void = allocate_ctx_regs();
        let dtor: fn(*mut c_void) = |_| {};

        (data, dtor)
    }

    #[test]
    fn init_ctx_regs() {
        // let (data, _dtor) = state_creator();
        //
        // let regs = wasmer_ctx_data_regs!(data);
        //
        // let reg0: &mut WasmerReg64 = ctx_regs_reg!(regs, 0);
        //
        // dbg!(reg0);

        // let reg1 = reg0.clone();
        // let reg2 = reg1;
        // let reg3 = reg2;
        //
        // dbg!(reg0);
        // dbg!(reg1);
        // dbg!(reg3);
        // dbg!(reg2);

        // assert_eq!(&mut WasmerReg64::new(), reg0);

        // let reg1 = ctx_regs_reg!(regs, 0);
    }
}
