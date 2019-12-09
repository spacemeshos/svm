use crate::ctx::SvmCtx;
use std::ffi::c_void;

#[inline(always)]
fn cast_wasmer_data_to_svm_ctx(data: *const c_void) -> *mut SvmCtx {
    data as _
}

#[inline(always)]
fn wasmer_data_node_data(data: *const c_void) -> *const c_void {
    let ctx = cast_wasmer_data_to_svm_ctx(data);
    let ctx: &mut SvmCtx = unsafe { &mut *ctx };

    ctx.node_data
}

/// Casts a `wasmer` instance's `data` field (of type: `c_void`) into `SvmContext`
#[macro_export]
macro_rules! cast_wasmer_data_to_svm_ctx {
    ($data: expr) => {{
        use $crate::ctx::SvmCtx;

        let ctx_ptr = $data as *mut SvmCtx;
        let ctx: &mut SvmCtx = unsafe { &mut *ctx_ptr };

        ctx
    }};
}

/// Extracts from `wasmer` instance context `data` (type: `SvmCtx`) the `node_data` field (type: `*const c_void`)
#[macro_export]
macro_rules! wasmer_data_node_data {
    ($data: expr) => {{
        use $crate::ctx::SvmCtx;
        let ctx: &mut SvmCtx = $crate::cast_wasmer_data_to_svm_ctx!($data);

        ctx.node_data
    }};
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
