// SVM imports
svm_extern::include_storage_vmcalls!();
svm_extern::include_buffer_vmcalls!();
svm_extern::include_host_ctx_vmcalls!();
svm_extern::include_register_vmcalls!();

// Host imports
extern "C" {
    /// Transfering `amount` coins from app's account
    /// into address given via register `{reg_bits}:{reg_idx}`
    fn host_transfer(amount: u64, reg_bits: u32, reg_idx: u32);
}
