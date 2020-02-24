/// This macro includes all the register vmcalls.
#[macro_export]
macro_rules! include_register_vmcalls {
    () => {
        extern "C" {
            fn reg_push(reg_bits: i32, reg_idx: i32);

            fn reg_pop(reg_bits: i32, reg_idx: i32);

            fn reg_eql(reg_bits: u32, src_idx: u32, dst_idx: u32) -> u32;
        }
    };
}
