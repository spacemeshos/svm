use std::ffi::c_void;

use crate::ctx::SvmCtx;
use crate::register::SvmReg;

use wasmer_runtime_core::vm::Ctx;

/// Given register bits count, returns the number of allocated registers of that type
/// (constants are defined at `ctx.rs`)
/// Ensuring that `reg_idx` is within the `0..REGS_64_COUNT` range (exclusive).
#[inline(always)]
pub fn wasmer_data_ensure_reg_idx(bits_count: i32, reg_idx: i32) {
    assert!(reg_idx >= 0 && ((reg_idx as i32) < (regs_count_ident(bits_count) as i32)));
}

/// Receives an slice of `SvmReg` and returns the `reg_idx` register.
pub fn svm_regs_reg(regs: &mut [SvmReg], bits_count: i32, reg_idx: i32) -> &mut SvmReg {
    wasmer_data_ensure_reg_idx(bits_count, reg_idx);

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

/// Extracts from `wasmer` instance context data field (of type `*mut c_void`), a mutable borrow for the register indexed `reg_idx`.
pub fn wasmer_data_reg<'a>(data: *const c_void, bits_count: i32, reg_idx: i32) -> &'a mut SvmReg {
    wasmer_data_ensure_reg_idx(bits_count, reg_idx);

    let ctx: &mut SvmCtx = crate::helpers::cast_wasmer_data_to_svm_ctx(data);

    match bits_count {
        32 => svm_regs_reg(&mut ctx.regs_32, 32, reg_idx),
        64 => svm_regs_reg(&mut ctx.regs_64, 64, reg_idx),
        160 => svm_regs_reg(&mut ctx.regs_160, 160, reg_idx),
        256 => svm_regs_reg(&mut ctx.regs_256, 256, reg_idx),
        512 => svm_regs_reg(&mut ctx.regs_512, 512, reg_idx),
        _ => unreachable!(),
    }
}

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[inline(always)]
pub fn wasmer_ctx_reg(ctx: &mut Ctx, bits_count: i32, reg_idx: i32) -> &mut SvmReg {
    wasmer_data_reg(ctx.data, bits_count, reg_idx)
}

#[inline(always)]
fn regs_count_ident(bits_count: i32) -> usize {
    match bits_count {
        32 => crate::ctx::REGS_32_COUNT,
        64 => crate::ctx::REGS_64_COUNT,
        160 => crate::ctx::REGS_160_COUNT,
        256 => crate::ctx::REGS_256_COUNT,
        512 => crate::ctx::REGS_512_COUNT,
        _ => unreachable!(),
    }
}
