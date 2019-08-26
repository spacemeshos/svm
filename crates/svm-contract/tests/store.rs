use svm_common::Address;
use svm_contract::env::ContractEnv;
use svm_contract::memory::{MemContractStore, MemoryEnv};
use svm_contract::traits::ContractStore;

use svm_contract::*;

#[test]
fn store_wasm_contract() {
    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let origin = <MemoryEnv as ContractEnv>::build_contract(&bytes).unwrap();

    let store = MemContractStore::new();
    let mut env = MemoryEnv::new(store);

    env.store_contract(&origin);

    let store = env.get_store();
    let addr: Address = origin.address.unwrap();

    let stored = store.load(addr).unwrap();
    assert_eq!(stored, origin);
}
