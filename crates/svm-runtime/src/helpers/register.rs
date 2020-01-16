use std::ffi::c_void;

use crate::{ctx::SvmCtx, register::SvmReg};

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[inline(always)]
pub fn wasmer_data_reg<'a>(data: *mut c_void, reg_bits: i32, reg_idx: i32) -> &'a mut SvmReg {
    ensure_reg_index(reg_bits, reg_idx);

    let ctx: &mut SvmCtx = unsafe { svm_common::from_raw_mut::<SvmCtx>(data) };

    match reg_bits {
        32 => svm_regs_reg(&mut ctx.regs_32, 32, reg_idx),
        64 => svm_regs_reg(&mut ctx.regs_64, 64, reg_idx),
        160 => svm_regs_reg(&mut ctx.regs_160, 160, reg_idx),
        256 => svm_regs_reg(&mut ctx.regs_256, 256, reg_idx),
        512 => svm_regs_reg(&mut ctx.regs_512, 512, reg_idx),
        _ => unreachable!(),
    }
}

#[inline(always)]
fn regs_count_by_bits(reg_bits: i32) -> i32 {
    let reg_bits = match reg_bits {
        32 => crate::ctx::REGS_32_COUNT,
        64 => crate::ctx::REGS_64_COUNT,
        160 => crate::ctx::REGS_160_COUNT,
        256 => crate::ctx::REGS_256_COUNT,
        512 => crate::ctx::REGS_512_COUNT,
        _ => unreachable!(),
    };

    reg_bits as i32
}

/// Receives an slice of `SvmReg` and returns the `reg_idx` register.
fn svm_regs_reg(regs: &mut [SvmReg], reg_bits: i32, reg_idx: i32) -> &mut SvmReg {
    ensure_reg_index(reg_bits, reg_idx);

    // We don't do:
    // ```rust
    // let reg: &mut SvmReg64 = $regs.regs_64[$reg_idx as usize];
    // ```
    //
    // Because we like to keep the option to mutate a couple of registers simultaneously
    // without the Rust borrow checker getting angry...
    // so instead we use _Unsafe Rust_
    let regs_ptr: *mut SvmReg = regs.as_mut_ptr();

    let reg_idx_ptr = unsafe { regs_ptr.offset(reg_idx as isize) };
    let reg: &mut SvmReg = unsafe { &mut *reg_idx_ptr };

    reg
}

/// Given register bits count, returns the number of allocated registers of that type
/// (constants are defined at `ctx.rs`)
/// Ensuring that `reg_idx` is within the `0..REGS_64_COUNT` range (exclusive).
#[inline(always)]
fn ensure_reg_index(reg_bits: i32, reg_idx: i32) {
    assert!(reg_idx >= 0 && (reg_idx < regs_count_by_bits(reg_bits)));
}

/// Allocates registers. This macro is called at `SvmCtx` ctor.
/// The macro consists of essentially code duplication. in order to avoid using alternatives like
/// having `SvmRegXXX` implement the `Copy` marker or by using Unsafe Rust.
#[macro_export]
macro_rules! alloc_regs {
    (32, REGS_32_COUNT) => {{
        [
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
            SvmReg::Reg32(SvmReg32::new()),
        ]
    }};
    (64, REGS_64_COUNT) => {{
        [
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
        ]
    }};
    (160, REGS_160_COUNT) => {{
        [
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
            SvmReg::Reg160(SvmReg160::new()),
        ]
    }};
    (256, REGS_256_COUNT) => {{
        [
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
            SvmReg::Reg64(SvmReg64::new()),
        ]
    }};
    (512, REGS_512_COUNT) => {{
        [
            SvmReg::Reg512(SvmReg512::new()),
            SvmReg::Reg512(SvmReg512::new()),
            SvmReg::Reg512(SvmReg512::new()),
            SvmReg::Reg512(SvmReg512::new()),
        ]
    }};
}
