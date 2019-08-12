/// `*const svm_address_t` is a raw pointer to a Rust `Address` struct.
#[repr(C)]
pub struct svm_address_t;

/// `*const svm_receipt_t` is a raw pointer to a Rust `Receipt` struct.
#[repr(C)]
pub struct svm_receipt_t;

/// Deallocates the memory of the `addr`
fn svm_address_destroy(addr: *const svm_address_t) {
    unimplemented!()
}

/// Deallocates the memory of the `receipt`
fn svm_receipt_destroy(receipt: *const svm_receipt_t) {
    unimplemented!()
}
