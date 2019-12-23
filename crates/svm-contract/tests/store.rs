use svm_common::Address;

use svm_contract::{
    build::WireContractBuilder,
    env::ContractEnv,
    memory::{MemContractStore, MemoryEnv},
    traits::ContractStore,
};

#[test]
fn store_contract() {
    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let store = MemContractStore::new();
    let mut env = MemoryEnv::new(store);

    let contract = env.build_contract(&bytes).unwrap();
    let addr = env.compute_address(&contract);

    env.store_contract(&contract, &addr);

    let store = env.get_store();

    let stored = store.load(&addr).unwrap();
    assert_eq!(stored, contract);
}
