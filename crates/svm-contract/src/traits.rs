use crate::types::CodeHash;
use crate::wasm::Contract;

use svm_common::Address;

pub trait ContractSerializer {
    fn serialize(contract: &Contract) -> Vec<u8>;
}

pub trait ContractDeserializer {
    fn deserialize(bytes: Vec<u8>) -> Contract;
}

pub trait ContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    fn store(&mut self, contract: &Contract, hash: CodeHash);

    fn load(&self, address: Address) -> Option<Contract>;

    fn close(&mut self);
}

pub trait ContractAddressCompute {
    fn compute(contract: &Contract) -> Address;
}

pub trait ContractCodeHasher {
    fn hash(bytes: &[u8]) -> CodeHash;
}
