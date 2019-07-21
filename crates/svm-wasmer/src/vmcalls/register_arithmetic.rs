macro_rules! include_wasmer_svm_vmcalls {
    ($PC: ident) => {
        /// Doing an *unsigned* comparison between register `reg1` and register `reg2`.
        /// Assumption: register data is stored in Little-Endian encoding.
        ///
        /// Returns:
        /// `1`  - if `reg1 > reg2`
        /// `0`  - if `reg1 == reg2`
        /// `-1` - if `reg1 < reg2`
        fn register_ucmp(ctx: &mut wasmer_runtime::Ctx, reg1: i32, reg2: i32) -> i32 {
            use crate::register::WasmerReg64;

            let reg1: &WasmerReg64 = wasmer_data_reg!(ctx.data, reg1, $PC);
            let reg2: &WasmerReg64 = wasmer_data_reg!(ctx.data, reg2, $PC);

            // reg1.ucmp(reg2)
            0
        }

        fn register_ucmp_i32(ctx: &mut wasmer_runtime::Ctx, reg: i32, val: i32) -> i32 {
            let reg = wasmer_data_reg!(ctx.data, reg, $PC);
            0
        }

        fn register_ucmp_i64(ctx: &mut wasmer_runtime::Ctx, reg: i32, val: i64) -> i32 {
            let reg = wasmer_data_reg!(ctx.data, reg, $PC);
            0
        }

        fn register_uadd_i32(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg: i32,
            val: i32,
            dst_reg: i32,
        ) -> i32 {
            let src_reg = wasmer_data_reg!(ctx.data, src_reg, $PC);
            let drc_reg = wasmer_data_reg!(ctx.data, dst_reg, $PC);
            0
        }

        fn register_uadd_i64(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg: i32,
            val: i32,
            dst_reg: i32,
        ) -> i32 {
            let src_reg = wasmer_data_reg!(ctx.data, src_reg, $PC);
            let drc_reg = wasmer_data_reg!(ctx.data, dst_reg, $PC);
            0
        }

        fn register_usub_i32(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg: i32,
            val: i32,
            dst_reg: i32,
        ) -> i32 {
            let src_reg = wasmer_data_reg!(ctx.data, src_reg, $PC);
            let drc_reg = wasmer_data_reg!(ctx.data, dst_reg, $PC);
            0
        }

        fn register_usub_i64(
            ctx: &mut wasmer_runtime::Ctx,
            src_reg: i32,
            val: i32,
            dst_reg: i32,
        ) -> i32 {
            let src_reg = wasmer_data_reg!(ctx.data, src_reg, $PC);
            let drc_reg = wasmer_data_reg!(ctx.data, dst_reg, $PC);
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_storage::{default::DefaultPageCache, memory::MemPages, PageSliceCache};

    pub type MemPageCache<'pc, K = [u8; 32]> = DefaultPageCache<'pc, MemPages<K>>;

    include_wasmer_svm_vmcalls!(MemPageCache);

    #[test]
    fn test_register_ucmp() {}
}
