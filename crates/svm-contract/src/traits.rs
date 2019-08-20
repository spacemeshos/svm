use crate::types::CodeHash;
use crate::wasm::WasmContract;
use svm_common::Address;

pub trait CodeHashStore {
    fn store(&mut self, code: &[u8], hash: CodeHash);

    fn load(&self, hash: CodeHash) -> Option<Vec<u8>>;
}

pub trait ContractAddressCompute {
    fn compute(contract: &WasmContract) -> Address;
}
