/// Casts the `wasmer` instance context data field (of type `*mut c_void`) into `&mut [WasmerSvmReg64; REGS_64_COUNT]`
#[macro_export]
macro_rules! wasmer_data_regs {
    ($data: expr, $PC: ident) => {{
        use $crate::ctx::SvmCtx;
        use $crate::register::WasmerSvmReg64;

        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        &mut ctx.regs_64
    }};
}

/// Ensuring that `reg_idx` is within the `0..REGS_64_COUNT` range (exclusive).
#[macro_export]
macro_rules! wasmer_data_ensure_reg_idx {
    ($reg_idx: expr) => {{
        use $crate::ctx::REGS_64_COUNT;
        assert!($reg_idx >= 0 && (($reg_idx as i32) < (REGS_64_COUNT as i32)));
    }};
}

/// Receives an array of `WasmerSvmReg64` and returns the `reg_idx` register.
#[macro_export]
macro_rules! svm_regs_reg {
    ($regs: expr, $reg_idx: expr) => {{
        wasmer_data_ensure_reg_idx!($reg_idx);

        // We don't do:
        // ```rust
        // let reg: &mut WasmerSvmReg64 = $regs.regs_64[$reg_idx as usize];
        // ```
        //
        // Because we like to keep the option to  mutate a couple of registers simultaneously
        // without the Rust borrow checker getting angry...
        // so instead we use _Unsafe Rust_
        use $crate::register::WasmerSvmReg64;
        let regs_ptr: *mut WasmerSvmReg64 = $regs.as_mut_ptr();

        let reg_idx_ptr: *mut WasmerSvmReg64 = unsafe { regs_ptr.offset($reg_idx as isize) };
        let reg: &mut WasmerSvmReg64 = unsafe { &mut *reg_idx_ptr };

        reg
    }};
}

/// Extracts from `wasmer` instance context data field (of type `*mut c_void`), a mutable borrow for the register indexed `reg_idx`.
#[macro_export]
macro_rules! wasmer_data_reg {
    ($data: expr, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_ensure_reg_idx!($reg_idx);

        use $crate::ctx::SvmCtx;
        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        svm_regs_reg!(ctx.regs_64, $reg_idx)
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg {
    ($ctx: expr, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_reg!($ctx.data, $reg_idx, $PC)
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) the register indexed `reg_idx` and calls
/// on it `set` with input `data`.  Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg_write {
    ($ctx: expr, $reg_idx: expr, $data: expr, $PC: ident) => {{
        let reg = wasmer_data_reg!($ctx.data, $reg_idx, $PC);
        reg.set($data);
    }};
}
