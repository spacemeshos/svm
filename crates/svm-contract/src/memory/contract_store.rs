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
    /// Create a new store
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
    fn store(&mut self, contract: &Contract, addr: &Address, hash: CodeHash) {
        let serialized: Vec<u8> = S::serialize(contract);

        self.contract_bytes.insert(hash, serialized);
        self.addr_codehash.insert(addr.clone(), hash);
    }

    fn load(&self, addr: &Address) -> Option<Contract> {
        match self.addr_codehash.get(addr) {
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
