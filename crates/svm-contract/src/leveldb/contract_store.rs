use crate::traits::{ContractDeserializer, ContractSerializer, ContractStore};
use crate::types::CodeHash;
use crate::wasm::Contract;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

use svm_common::Address;
use svm_kv::leveldb::LDBStore;
use svm_kv::traits::KVStore;

pub struct LDBContractStore<S, D> {
    db: LDBStore,
    marker: PhantomData<(S, D)>,
}

impl<S, D> LDBContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    pub fn new(path: &Path) -> Self {
        Self {
            db: LDBStore::new(path),
            marker: PhantomData,
        }
    }
}

impl<S, D> ContractStore<S, D> for LDBContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    fn store(&mut self, contract: &Contract, hash: CodeHash) {
        let serialized: Vec<u8> = S::serialize(contract);
        let addr = contract.address.unwrap();

        let addr_hash = (addr.as_slice(), &hash.0[..]);
        let hash_wasm = (&hash.0[..], &serialized[..]);
        self.db.store(&[addr_hash, hash_wasm]);
    }

    fn load(&self, address: Address) -> Option<Contract> {
        match self.db.get(address.as_slice()) {
            None => None,
            Some(hash) => match self.db.get(&hash) {
                None => panic!(format!(
                    "code associated with `CodeHash = {:?}` not found",
                    hash
                )),
                Some(bytes) => {
                    let contract = D::deserialize(bytes.to_vec());
                    Some(contract)
                }
            },
        }
    }

    fn close(&mut self) {}
}
