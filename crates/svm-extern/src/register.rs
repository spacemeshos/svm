/// This macro includes all the register-arithmetic vmcalls (`extern "C"` interface);
#[macro_export]
macro_rules! include_extern_register_vmcalls {
    () => {
        extern "C" {
            fn register_le_ucmp(reg1: i32, reg2: i32) -> i32;

            fn register_le_ucmp_u64(reg: i32, val: i64) -> i32;

            fn register_le_uadd_u64(src_reg: i32, val: i64, dst_reg: i32) -> i32;

            fn register_le_usub_u64(src_reg: i32, val: i64, dst_reg: i32) -> i32;
        }
    };
}
