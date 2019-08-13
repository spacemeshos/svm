/// Casts a `wasmer` instance's `data` field (of type: `c_void`) into `SvmContext<PC>` (`PC` implements `PageCache`)
#[macro_export]
macro_rules! cast_wasmer_data_to_svm_ctx {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::SvmCtx;

        let ctx_ptr = $data as *mut SvmCtx<$PC>;
        let ctx: &mut SvmCtx<$PC> = unsafe { &mut *ctx_ptr };

        ctx
    }};
}

/// Extracts from `wasmer` instance context `data` (type: `SvmCtx`) the `node_data` field (type: `*const c_void`)
#[macro_export]
macro_rules! wasmer_data_node_data {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::SvmCtx;
        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        ctx.node_data
    }};
}

#[macro_export]
macro_rules! alloc_regs {
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
