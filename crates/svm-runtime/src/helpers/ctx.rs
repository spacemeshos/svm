use crate::ctx::SvmCtx;
use std::ffi::c_void;

#[inline(always)]
pub fn cast_ptr_to_svm_ctx<'a>(data: *const c_void) -> &'a mut SvmCtx {
    unsafe { &mut *(data as *mut SvmCtx) }
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
