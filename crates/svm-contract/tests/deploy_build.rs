use svm_common::Address;

use svm_contract::build::WireContractBuilder;
use svm_contract::{
    env::ContractEnv,
    memory::{MemContractStore, MemoryEnv},
};

#[test]
fn build_contract() {
    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let store = MemContractStore::new();
    let env = MemoryEnv::new(store);

    let contract = env.build_contract(&bytes).unwrap();

    assert_eq!("Contract #1", contract.name);
    assert_eq!(Address::from(0x10_20_30_40), contract.author);
    assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], contract.wasm.as_ref());
}
