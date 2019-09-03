use crate::traits::{ContractDeserializer, ContractSerializer, ContractStore};
use crate::types::CodeHash;
use crate::wasm::Contract;

use std::collections::HashMap;
use std::marker::PhantomData;

use svm_common::Address;

pub struct LDBContractStore<S, D> {
    marker: PhantomData<(S, D)>,
}

#[allow(dead_code)]
impl<S, D> LDBContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    pub fn new() -> Self {
        Self {
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

        // self.contract_bytes.insert(hash, serialized);

        let addr = contract.address.unwrap();
        // self.addr_codehash.insert(addr, hash);
    }

    fn load(&self, address: Address) -> Option<Contract> {
        unimplemented!()
        // match self.addr_codehash.get(&address) {
        //     None => None,
        //     Some(hash) => match self.contract_bytes.get(&hash) {
        //         None => panic!(format!(
        //             "Code associated with `CodeHash = {:?}` not found",
        //             hash
        //         )),
        //         Some(bytes) => {
        //             let contract = D::deserialize(bytes.to_vec());
        //             Some(contract)
        //         }
        //     }
        // }
    }

    fn close(&mut self) {}
}
