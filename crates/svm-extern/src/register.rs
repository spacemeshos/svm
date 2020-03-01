/// This macro includes all the register vmcalls.
#[macro_export]
macro_rules! include_register_vmcalls {
    () => {
        extern "C" {
            fn reg_push(reg_bits: u32, reg_idx: u32);

            fn reg_pop(reg_bits: u32, reg_idx: u32);

            fn reg_cmp(reg_bits: u32, src_idx: u32, dst_idx: u32) -> i32;

            fn reg_set_i32_be(reg_bits: u32, reg_idx: u32, n: u32);

            fn reg_set_i32_le(reg_bits: u32, reg_idx: u32, n: u32);

            fn reg_set_i64_be(reg_bits: u32, reg_idx: u32, n: u64);

            fn reg_set_i64_le(reg_bits: u32, reg_idx: u32, n: u64);
        }
    };
}
