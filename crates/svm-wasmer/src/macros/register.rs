#[macro_export]
macro_rules! regs_count_ident {
    (64) => {
        $crate::ctx::REGS_64_COUNT;
    };
    (160) => {
        $crate::ctx::REGS_160_COUNT;
    };
    (256) => {
        $crate::ctx::REGS_256_COUNT;
    };
    (512) => {
        $crate::ctx::REGS_512_COUNT;
    };
}

#[macro_export]
macro_rules! reg_type_ident {
    (64) => {
        $crate::register::SvmReg64
    };
    (160) => {
        $crate::register::SvmReg160
    };
    (256) => {
        $crate::register::SvmReg256
    };
    (512) => {
        $crate::register::SvmReg512
    };
}

#[macro_export]
macro_rules! svm_ctx_regs_var {
    ($ctx: expr, 64) => {
        $ctx.regs_64
    };
    ($ctx: expr, 160) => {
        $ctx.regs_160
    };
    ($ctx: expr, 256) => {
        $ctx.regs_256
    };
    ($ctx: expr, 512) => {
        $ctx.regs_512
    };
}

/// Ensuring that `reg_idx` is within the `0..REGS_64_COUNT` range (exclusive).
#[macro_export]
macro_rules! wasmer_data_ensure_reg_idx {
    (64, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && (($reg_idx as i32) < (regs_count_ident!(64) as i32)));
    }};
    (160, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && (($reg_idx as i32) < (regs_count_ident!(160) as i32)));
    }};
    (256, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && (($reg_idx as i32) < (regs_count_ident!(256) as i32)));
    }};
    (512, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && (($reg_idx as i32) < (regs_count_ident!(512) as i32)));
    }};
}

/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut [SvmReg64; REGS_64_COUNT]`
#[macro_export]
macro_rules! wasmer_data_regs {
    ($data: expr, 64, $PC: ident) => {{
        use $crate::ctx::SvmCtx;

        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);
        &mut svm_ctx_regs_var!(ctx, 64)
    }};
}

/// Receives an array of `SvmReg64` and returns the `reg_idx` register.
#[macro_export]
macro_rules! svm_regs_reg {
    ($regs: expr, 64, $reg_idx: expr) => {{
        wasmer_data_ensure_reg_idx!(64, $reg_idx);

        // We don't do:
        // ```rust
        // let reg: &mut SvmReg64 = $regs.regs_64[$reg_idx as usize];
        // ```
        //
        // Because we like to keep the option to  mutate a couple of registers simultaneously
        // without the Rust borrow checker getting angry...
        // so instead we use _Unsafe Rust_
        use $crate::register::SvmReg;
        let regs_ptr: *mut SvmReg = $regs.as_mut_ptr();

        let reg_idx_ptr = unsafe { regs_ptr.offset($reg_idx as isize) };
        let reg: &mut SvmReg = unsafe { &mut *reg_idx_ptr };

        reg
    }};
}

/// Extracts from `wasmer` instance context data field (of type `*mut c_void`), a mutable borrow for the register indexed `reg_idx`.
#[macro_export]
macro_rules! wasmer_data_reg {
    ($data: expr, 64, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_ensure_reg_idx!(64, $reg_idx);

        use $crate::ctx::SvmCtx;
        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        svm_regs_reg!(svm_ctx_regs_var!(ctx, 64), 64, $reg_idx)
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg {
    ($ctx: expr, 64, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_reg!($ctx.data, 64, $reg_idx, $PC)
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) the register indexed `reg_idx` and calls
/// on it `set` with input `data`.  Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg_write {
    ($ctx: expr, 64, $reg_idx: expr, $data: expr, $PC: ident) => {{
        let reg = wasmer_data_reg!($ctx.data, 64, $reg_idx, $PC);
        reg.set($data);
    }};
}
