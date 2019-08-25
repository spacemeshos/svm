use crate::traits::ContractAddressCompute;
use crate::wasm::WasmContract;
use svm_common::{Address, DefaultKeyHasher, KeyHasher};

pub struct DefaultContractAddressCompute;

impl ContractAddressCompute for DefaultContractAddressCompute {
    fn compute(contract: &WasmContract) -> Address {
        // Computing the contract's account address as follows:
        // First 32 bytes of HASH(contract.Author || contract.Wasm)

        let wasm_len = contract.wasm.len();
        let author_len = contract.author.len();
        let key = vec![0; author_len + wasm_len];

        let mut hash = DefaultKeyHasher::hash(&key);

        // `Address::from` expects input in Little-Endian order.
        // so we reverse `hash` first
        hash.reverse();

        Address::from(hash.as_ref())
    }
}
