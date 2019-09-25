use crate::traits::{ContractDeserializer, ContractSerializer, ContractStore};
use crate::types::CodeHash;
use crate::wasm::Contract;

use std::marker::PhantomData;
use std::path::Path;

use svm_common::Address;
use svm_kv::rocksdb::RocksStore;
use svm_kv::traits::KVStore;

use log::info;

pub struct RocksContractStore<S, D> {
    db: RocksStore,
    marker: PhantomData<(S, D)>,
}

impl<S, D> RocksContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    pub fn new(path: &Path) -> Self {
        Self {
            db: RocksStore::new(path),
            marker: PhantomData,
        }
    }
}

impl<S, D> ContractStore<S, D> for RocksContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    fn store(&mut self, contract: &Contract, addr: &Address, hash: CodeHash) {
        info!("storing contract: \n{:?}", contract);
        info!("     contract account address: {:?}", addr);
        info!("     contract code-hash: {:?}", hash);

        let serialized: Vec<u8> = S::serialize(contract);

        let addr_hash = (addr.as_slice(), &hash.0[..]);
        let hash_wasm = (&hash.0[..], &serialized[..]);
        self.db.store(&[addr_hash, hash_wasm]);
    }

    fn load(&self, addr: &Address) -> Option<Contract> {
        info!("loading contract account {:?}", addr);

        match self.db.get(addr.as_slice()) {
            None => None,
            Some(hash) => match self.db.get(&hash) {
                None => panic!(format!(
                    "code associated with `CodeHash = {:?}` not found",
                    hash
                )),
                Some(bytes) => {
                    let contract = D::deserialize(bytes.to_vec());
                    info!("loaded contract: \n{:?}", contract);

                    Some(contract)
                }
            },
        }
    }
}
