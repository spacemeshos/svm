use crate::traits::{ContractDeserializer, ContractSerializer, ContractStore};
use crate::types::CodeHash;
use crate::wasm::Contract;

use std::collections::HashMap;
use std::marker::PhantomData;

use svm_common::Address;

/// An in-memory implementation of `ContractStore`
pub struct MemContractStore<S, D> {
    contract_bytes: HashMap<CodeHash, Vec<u8>>,
    addr_codehash: HashMap<Address, CodeHash>,
    marker: PhantomData<(S, D)>,
}

#[allow(dead_code)]
impl<S, D> MemContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            contract_bytes: HashMap::new(),
            addr_codehash: HashMap::new(),
            marker: PhantomData,
        }
    }
}

impl<S, D> ContractStore<S, D> for MemContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    fn store(&mut self, contract: &Contract, hash: CodeHash) {
        let serialized: Vec<u8> = S::serialize(contract);

        self.contract_bytes.insert(hash, serialized);

        let addr = contract.address.unwrap();
        self.addr_codehash.insert(addr, hash);
    }

    fn load(&self, address: Address) -> Option<Contract> {
        match self.addr_codehash.get(&address) {
            None => None,
            Some(hash) => match self.contract_bytes.get(&hash) {
                None => panic!(format!(
                    "Code associated with `CodeHash = {:?}` not found",
                    hash
                )),
                Some(bytes) => {
                    let contract = D::deserialize(bytes.to_vec());
                    Some(contract)
                }
            },
        }
    }
}
