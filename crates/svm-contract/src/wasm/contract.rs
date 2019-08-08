use svm_common::Address;

#[allow(dead_code)]
pub struct WasmContract {
    pub(crate) address: Address,
    pub(crate) wasm: Vec<u8>,
    pub(crate) author: Address,
    pub(crate) owners: Vec<Address>,
}
