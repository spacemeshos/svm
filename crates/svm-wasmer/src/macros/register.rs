/// Given regiser bits count, returns the number of allocated registers of that type
/// (constants are defined at `ctx.rs`)
#[macro_export]
macro_rules! regs_count_ident {
    ($bits_count: expr) => {{
        match $bits_count {
            32 => $crate::ctx::REGS_32_COUNT,
            64 => $crate::ctx::REGS_64_COUNT,
            160 => $crate::ctx::REGS_160_COUNT,
            256 => $crate::ctx::REGS_256_COUNT,
            512 => $crate::ctx::REGS_512_COUNT,
            _ => unreachable!(),
        }
    }};
}

/// Ensuring that `reg_idx` is within the `0..REGS_64_COUNT` range (exclusive).
#[macro_export]
macro_rules! wasmer_data_ensure_reg_idx {
    ($bits_count: expr, $reg_idx: expr) => {{
        assert!($reg_idx >= 0 && (($reg_idx as i32) < (regs_count_ident!($bits_count) as i32)));
    }};
}

/// Receives an array of `SvmReg64` and returns the `reg_idx` register.
#[macro_export]
macro_rules! svm_regs_reg {
    ($regs: expr, $bits_count: expr, $reg_idx: expr) => {{
        wasmer_data_ensure_reg_idx!($bits_count, $reg_idx);

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
    ($data: expr, $bits_count: expr, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_ensure_reg_idx!($bits_count, $reg_idx);

        use $crate::ctx::SvmCtx;
        let ctx: &mut SvmCtx<$PC> = cast_wasmer_data_to_svm_ctx!($data, $PC);

        match $bits_count {
            32 => svm_regs_reg!(ctx.regs_32, 32, $reg_idx),
            64 => svm_regs_reg!(ctx.regs_64, 64, $reg_idx),
            160 => svm_regs_reg!(ctx.regs_160, 160, $reg_idx),
            256 => svm_regs_reg!(ctx.regs_256, 256, $reg_idx),
            512 => svm_regs_reg!(ctx.regs_512, 512, $reg_idx),
            _ => unreachable!(),
        }
    }};
}

/// Extracts from `wasmer` instance context (type: `Ctx`) a mutable borrow for the register indexed `reg_idx`.
/// Will be used by storage vmcalls.
#[macro_export]
macro_rules! wasmer_ctx_reg {
    ($ctx: expr, $bits_count: expr, $reg_idx: expr, $PC: ident) => {{
        wasmer_data_reg!($ctx.data, $bits_count, $reg_idx, $PC)
    }};
}
