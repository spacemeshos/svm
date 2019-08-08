#[repr(C)]
pub struct svm_address_t;

/// Deallocated the memory of the `addr`
fn svm_address_destroy(addr: *const svm_address_t) {
    unimplemented!()
}
