/// This macro includes all the register-arithmetic vmcalls (`extern "C"` interface);
#[macro_export]
macro_rules! include_extern_register_vmcalls {
    () => {
        extern "C" {
            fn reg_replace_byte(reg_bits: i32, reg_idx: i32, byte: i32, offset: i32);

            fn reg_read_le_i64(reg_bits: i32, reg_idx: i32) -> i64;

            fn reg_write_le_i64(value: i64, reg_bits: i32, reg_idx: i32);
        }
    };
}
