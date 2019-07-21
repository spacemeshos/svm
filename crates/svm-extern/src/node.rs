/// The node-related vmcalls will be imported by Smart Contracts that will want to integrate with the _Full-Node_ runtime.

/// This macro includes all the node vmcalls `extern "C"` interfaces.
#[macro_export]
macro_rules! include_extern_node_vmcalls {
    () => {
        extern "C" {
            /// * `src_account_reg` - The register index that holds the source node account we want to retrieve its balance.
            /// * `dst_balance_reg` - The register index we want to copy the account's balance into.
            fn get_balance_into_reg(src_account_reg: i32, dst_balance_reg: i32);

            /// * `src_account_reg` - The register index that holds the account address we want to set a new balance.
            /// * `src_balance_reg` - The register index that holds the balance amount we want to set.
            fn set_balance_from_reg(src_account_reg: i32, src_balance_reg: i32);
        }
    };
}
