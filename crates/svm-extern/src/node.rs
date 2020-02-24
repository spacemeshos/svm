/// This macro includes all the `node` vmcalls.
#[macro_export]
macro_rules! include_node_vmcalls {
    () => {
        extern "C" {
            fn get_balance(reg_bits: i32, reg_idx: i32) -> i64;

            fn set_balance(reg_bits: i32, reg_idx: i32, balance: i64);
        }
    };
}
