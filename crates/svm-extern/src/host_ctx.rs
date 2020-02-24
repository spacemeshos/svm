/// This macro includes all the host_ctx vmcalls.
#[macro_export]
macro_rules! include_host_ctx_vmcalls {
    () => {
        extern "C" {
            fn host_ctx_read_into_reg(field_idx: u32, reg_bits: u32, reg_idx: u32);

            fn host_ctx_read_i32_le(field_idx: u32) -> u32;

            fn host_ctx_read_i32_be(ctx: &mut WasmerCtx, field_idx: u32) -> u32;

            fn host_ctx_read_i64_le(ctx: &mut WasmerCtx, field_idx: u32) -> u64;

            fn host_ctx_read_i64_be(ctx: &mut WasmerCtx, field_idx: u32) -> u64;
        }
    };
}
