use crate::types::CodeHash;
use crate::wasm::WasmContract;

use svm_common::Address;

pub trait ContractStore {
    fn store(&mut self, contract: &WasmContract, hash: CodeHash, address: Address);

    fn load(&self, address: Address) -> Option<WasmContract>;
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
