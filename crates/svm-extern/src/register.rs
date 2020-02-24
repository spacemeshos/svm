/// This macro includes all the register vmcalls.
#[macro_export]
macro_rules! include_register_vmcalls {
    () => {
        extern "C" {
            fn reg_push(reg_bits: i32, reg_idx: i32);

            fn reg_pop(reg_bits: i32, reg_idx: i32);
        }
    };
}
