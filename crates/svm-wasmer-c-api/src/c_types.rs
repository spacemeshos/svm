/// `*const svm_address_t` is a raw pointer to a Rust `Address` struct.
#[repr(C)]
pub struct svm_address_t;

/// `*const svm_contract_ctx_t` is a raw pointer to the a Rust `ContractCtx` struct.
#[repr(C)]
pub struct svm_contract_ctx_t;

/// `*const svm_receipt_t` is a raw pointer to a Rust `Receipt` struct.
#[repr(C)]
pub struct svm_receipt_t;

/// Deallocated the memory of the `addr`
fn svm_address_destroy(addr: *const svm_address_t) {
    unimplemented!()
}
