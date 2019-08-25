use crate::traits::{ContractDeserializer, ContractSerializer, ContractStore};
use crate::types::CodeHash;
use crate::wasm::WasmContract;

use std::collections::HashMap;
use svm_common::Address;

pub struct MemContractStore {
    contract_bytes: HashMap<CodeHash, Vec<u8>>,
    addr_codehash: HashMap<Address, CodeHash>,
}

#[allow(dead_code)]
impl MemContractStore {
    fn new() -> Self {
        Self {
            contract_bytes: HashMap::new(),
            addr_codehash: HashMap::new(),
        }
    }
}

impl ContractStore for MemContractStore {
    fn store(&mut self, contract: &WasmContract, hash: CodeHash, address: Address) {
        // self.map.insert(hash, code.to_owned());
    }

    fn load(&self, address: Address) -> Option<WasmContract> {
        match self.addr_codehash.get(&address) {
            None => None,
            Some(hash) => match self.contract_bytes.get(&hash) {
                None => panic!(format!(
                    "Code associated with `CodeHash = {:?}` not found",
                    hash
                )),
                Some(bytes) => {
                    let contract = WasmContract::deserialize(bytes.to_vec());
                    Some(contract)
                }
            },
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn one_code() {
//         let mut store = MemContractStore::new();
//
//         let hash = CodeHash([10; 32]);
//         assert_eq!(None, store.load(hash));
//
//         // 1st store
//         store.store(&[10, 20, 30], hash);
//         assert_eq!(vec![10, 20, 30], store.load(hash).unwrap());
//
//         // 2nd store, no change
//         store.store(&[10, 20, 30], hash);
//         assert_eq!(vec![10, 20, 30], store.load(hash).unwrap());
//     }
//
//     #[test]
//     fn two_codes() {
//         let mut store1 = MemContractStore::new();
//         let mut store2 = MemContractStore::new();
//
//         let hash1 = CodeHash([10; 32]);
//         let hash2 = CodeHash([20; 32]);
//         assert_eq!(None, store1.load(hash1));
//         assert_eq!(None, store2.load(hash2));
//
//         store1.store(&[10, 20, 30], hash1);
//         store2.store(&[40, 50, 60], hash2);
//         assert_eq!(vec![10, 20, 30], store1.load(hash1).unwrap());
//         assert_eq!(vec![40, 50, 60], store2.load(hash2).unwrap());
//     }
// }
