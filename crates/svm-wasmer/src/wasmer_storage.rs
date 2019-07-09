use wasmer_runtime_core::vm::Ctx;

use super::wasmer_register::WasmerReg64;

/// The number of allocated `64-bit` wasmer registers for each `SvmCtx`
pub const REGS_64_COUNT: usize = 8;

#[repr(C)]
#[derive(Debug)]
/// `SvmCtx` is a container for the accessible data by `wasmer` instances
/// Its fields are:
/// * `regs_64` - an static array (`REGS_64_COUNT` elements) of `WasmerReg64`
pub struct SvmCtx {
    pub(crate) regs_64: [WasmerReg64; REGS_64_COUNT],
}

impl SvmCtx {
    /// Initializes a new empty `SvmCtx`
    pub fn new() -> Self {
        let regs_64 = [WasmerReg64::new(); REGS_64_COUNT];

        Self { regs_64 }
    }
}

macro_rules! ctx_regs_reg {
    ($regs: expr, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && $reg_idx < REGS_64_COUNT as i32);

        /// We don't do:
        /// ```rust
        /// let reg: &mut WasmerReg64 = $regs.regs_64[$reg_idx as usize];
        /// ```
        ///
        /// Because we like to keep the option to  mutate a couple of registers simultaneously
        /// without the Rust borrow checker getting angry...
        /// so instead we use _Unsafe Rust_
        let regs_ptr: *mut WasmerReg64 = $regs.regs_64.as_mut_ptr();

        let reg_idx_ptr: *mut WasmerReg64 = unsafe { regs_ptr.offset($reg_idx as isize) };

        let reg: &mut WasmerReg64 = unsafe { &mut *reg_idx_ptr };

        reg
    }};
}

macro_rules! wasmer_ctx_data_regs {
    ($data: expr) => {{
        let data_ptr: *mut SvmCtx = $data as *mut _;
        let data: &mut SvmCtx = unsafe { &mut *data_ptr };
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

/// Copies the content of `wasmer` memory cells under addresses:
/// `src_mem_ptr, src_mem_ptr + 1, .. , src_mem_ptr + len (exclusive)`
/// into `wasmer` register indexed `dst_reg`
pub fn mem_to_reg_copy(ctx: &mut Ctx, src_mem_ptr: i32, dst_reg: i32, len: i32) {
    let reg = wasmer_reg!(ctx, dst_reg);
    let cells = wasmer_mem_cells!(ctx, src_mem_ptr, len);

    reg.copy_from_wasmer_mem(cells);
}

/// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
/// `dst_mem_ptr, dst_mem_ptr + 1, .. , dst_mem_ptr + len (exclusive)`
pub fn reg_to_mem_copy(ctx: &mut Ctx, src_reg: i32, dst_mem_ptr: i32, len: i32) {
    let reg = wasmer_reg!(ctx, src_reg);
    let cells = wasmer_mem_cells!(ctx, dst_mem_ptr, len);

    reg.copy_to_wasmer_mem(cells);
}

// #[allow(unused)]
// pub fn storage_read_to_reg(_ctx: &mut Ctx, _page: i32, _offset: i32, _len: i32, _reg: i32) {
//     //
// }
//
// #[allow(unused)]
// pub fn storage_set_from_reg(_ctx: &mut Ctx, _page: i32, _offset: i32, _reg: i32) {
//     //
// }

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::Cell;
    use std::ffi::c_void;

    fn state_creator(regs: &SvmCtx) -> (*mut c_void, fn(*mut c_void)) {
        let data: *mut c_void = regs.clone() as *const _ as *mut c_void;
        let dtor: fn(*mut c_void) = |_| {};

        (data, dtor)
    }

    #[test]
    fn reg_copy_from_wasmer_mem() {
        let regs = SvmCtx::new();

        let (data, _dtor) = state_creator(&regs);

        let regs = wasmer_ctx_data_regs!(data);
        let reg0: &mut WasmerReg64 = ctx_regs_reg!(regs, 0);
        let reg1: &mut WasmerReg64 = ctx_regs_reg!(regs, 1);

        // registers `0` and `1` are initialized with zeros
        assert_eq!(vec![0; 8], &reg0.0[..]);
        assert_eq!(vec![0; 8], &reg1.0[..]);

        // editing register `0` should not edit register `1`
        let cells = vec![
            Cell::new(10),
            Cell::new(20),
            Cell::new(30),
            Cell::new(40),
            Cell::new(50),
            Cell::new(60),
            Cell::new(70),
            Cell::new(80),
        ];

        reg0.copy_from_wasmer_mem(&cells);

        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], &reg0.0[..]);
        assert_eq!(vec![00, 00, 00, 00, 00, 00, 00, 00], &reg1.0[..]);
    }

    #[test]
    fn reg_copy_to_wasmer_mem() {
        let regs = SvmCtx::new();

        let (data, _dtor) = state_creator(&regs);

        let regs = wasmer_ctx_data_regs!(data);
        let reg0: &mut WasmerReg64 = ctx_regs_reg!(regs, 0);

        // initialize register `0` with data
        let cells = vec![
            Cell::new(10),
            Cell::new(20),
            Cell::new(30),
            Cell::new(40),
            Cell::new(50),
            Cell::new(60),
            Cell::new(70),
            Cell::new(80),
        ];

        reg0.copy_from_wasmer_mem(&cells);
        assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], &reg0.0[..]);

        // copying register `0` to a fake wasmer memory at cells: `10, 11, ... 17` (inclusive)
        let cells: Vec<Cell<u8>> = (0..8).map(|_| Cell::<u8>::new(0)).collect();

        assert_eq!(
            vec![00, 00, 00, 00, 00, 00, 00, 00],
            cells.iter().map(|c| c.get()).collect::<Vec<u8>>()
        );

        // copying register `0` content into memory
        reg0.copy_to_wasmer_mem(&cells);

        // asserint that the fake wasmer memory has been changes
        assert_eq!(
            vec![10, 20, 30, 40, 50, 60, 70, 80],
            cells.iter().map(|c| c.get()).collect::<Vec<u8>>()
        );
    }
}
