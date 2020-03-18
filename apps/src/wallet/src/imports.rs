// SVM imports
svm_extern::include_storage_vmcalls!();
svm_extern::include_buffer_vmcalls!();
svm_extern::include_host_ctx_vmcalls!();
svm_extern::include_register_vmcalls!();

// Host imports
extern "C" {
    /// Returns the current app's balance.
    fn host_get_my_balance() -> u64;

    /// Transferring `amount` coins from app's account
    /// into address given via register `{reg_bits}:{reg_idx}`
    fn host_transfer(amount: u64, reg_bits: u32, reg_idx: u32);
}
