/// The `include_extern_node_XXXX` vmcalls will be imported by Smart Contracts that will want to integrate with the Full-Node runtime.
/// We allow vmcalls picking granularity by wrapping each vmcall in a dedicated macro.
/// If you'd like to use an import-all vmcalls, then use `include_extern_node_all_vmcalls` macro

/// * `src_account_reg` - The register index that holds the source node account we want to retrieve its balance.
/// * `dst_balance_reg` - The register index we want to copy the account's balance into.
#[macro_export]
macro_rules! include_extern_node_get_balance_reg {
    () => {
        extern "C" {
            fn get_balance_into_reg(src_account_reg: i32, dst_balance_reg: i32);
        }
    };
}

/// * `src_account_reg` - The register index that holds the account address we want to set a new balance.
/// * `src_balance_reg` - The register index that holds the balance amount we want to set.
#[macro_export]
macro_rules! include_extern_node_set_balance_reg {
    () => {
        extern "C" {
            fn set_balance_from_reg(src_account_reg: i32, src_balance_reg: i32);
        }
    };
}

/// This macro includes all the node vmcalls `extern "C"` interfaces.
#[macro_export]
macro_rules! include_extern_node_all_vmcalls {
    () => {
        include_extern_node_get_balance_reg!();

        include_extern_node_set_balance_reg!();
    };
}
