use crate::types::CodeHash;
use crate::wasm::WasmContract;
use svm_common::Address;

pub trait CodeHashLocator {
    fn store(&mut self, hash: CodeHash, address: Address);

    fn exists(&self, hash: CodeHash) -> bool;
}

pub trait ContractLoader {
    fn load(address: Address) -> Vec<u8>;
}

pub trait ContractSerializer {
    fn serialize(contract: &WasmContract) -> Vec<u8>;
}

pub trait ContractDeserializer {
    fn deserialize(bytes: &[u8]) -> WasmContract;
}

pub trait ContractAddressCompute {
    fn compute(contract: &WasmContract) -> Address;
}
