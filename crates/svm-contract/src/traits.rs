use crate::types::CodeHash;
use crate::wasm::WasmContract;

use svm_common::Address;

pub trait ContractSerializer {
    fn serialize(contract: &WasmContract) -> Vec<u8>;
}

pub trait ContractDeserializer {
    fn deserialize(bytes: Vec<u8>) -> WasmContract;
}

pub trait ContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    fn store(&mut self, contract: &WasmContract);

    fn load(&self, address: Address) -> Option<WasmContract>;

    fn close(&mut self);
}

pub trait ContractAddressCompute {
    fn compute(contract: &WasmContract) -> Address;
}

pub trait ContractCodeHasher {
    fn hash(bytes: &[u8]) -> CodeHash;
}
