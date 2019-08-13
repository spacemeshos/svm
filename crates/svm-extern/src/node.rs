/// The node-related vmcalls will be imported by Smart Contracts that will want to integrate with the _Full-Node_ runtime.

/// This macro includes all the node vmcalls `extern "C"` interfaces.
#[macro_export]
macro_rules! include_extern_node_vmcalls {
    () => {
        extern "C" {
            /// * `reg_bits` - The #bits of `reg_idx`
            /// * `reg_idx`  - The register index that holds the source node account we want to retrieve its balance.
            fn get_balance_from_reg(reg_bits: i32, reg_idx: i32) -> i64;

            /// * `reg_bits` - The #bits of `reg_idx`
            /// * `reg_idx`  - The register index that holds the account address we want to set its balance
            /// * `balance`  - The new balance
            fn set_balance_from_reg(reg_bits: i32, reg_idx: i32, balance: i64);
        }
    };
}
