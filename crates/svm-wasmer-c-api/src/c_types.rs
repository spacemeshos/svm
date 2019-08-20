use svm_common::Address;

/// `*const svm_address_t` is a raw pointer to a Rust `Address` struct.
#[repr(C)]
pub struct svm_address_t;

/// `*const svm_wasm_contract_t` is a raw pointer to a Rust `WasmContract` struct.
#[repr(C)]
pub struct svm_wasm_contract_t;

/// `*const svm_receipt_t` is a raw pointer to a Rust `Receipt` struct.
#[repr(C)]
pub struct svm_receipt_t;

#[allow(unused)]
/// Deallocates the memory of the `addr`
fn svm_address_destroy(addr: *const svm_address_t) {
    let addr: *mut svm_address_t = addr as _;
    unsafe {
        Box::from_raw(addr as *mut Address);
    }
}

#[allow(unused)]
/// Deallocates the memory of the `receipt`
fn svm_wasm_contract_destroy(contract: *const svm_wasm_contract_t) {
    panic!();
    // let contract: *mut svm_wasm_contract_t = contract as _;
    // unsafe {
    //     Box::from_raw(contract as *mut WasmContract);
    // }
}

#[allow(unused)]
/// Deallocates the memory of the `receipt`
fn svm_receipt_destroy(receipt: *const svm_receipt_t) {
    panic!();
    // let receipt: *mut svm_receipt_t = receipt as _;
    // unsafe {
    //     Box::from_raw(receipt as *mut Receipt);
    // }
}
