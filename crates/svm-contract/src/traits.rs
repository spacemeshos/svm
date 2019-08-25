use crate::types::CodeHash;
use crate::wasm::WasmContract;

use svm_common::Address;

pub trait CodeHashStore {
    fn store(&mut self, core: &[u8], hash: CodeHash);

    fn load(&self, hash: CodeHash) -> Option<Vec<u8>>;

    fn exists(&self, hash: CodeHash) -> bool;
}

pub trait ContractAddressCompute {
    fn compute(contract: &WasmContract) -> Address;
}

pub trait ContractSerializer {
    fn serialize(contract: &WasmContract) -> Vec<u8>;
}

pub trait ContractDeserializer {
    fn deserialize(bytes: Vec<u8>) -> WasmContract;
}
