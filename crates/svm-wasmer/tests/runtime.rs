use svm_common::Address;
use svm_contract::env::ContractEnv;
use svm_contract::memory::{MemContractStore, MemoryEnv};
use svm_contract::WireContractBuilder;
use svm_storage::memory::MemMerklePageCache;
use svm_wasmer::*;

include_svm_runtime!(MemMerklePageCache, svm_contract::memory::MemoryEnv);

#[test]
fn deploy_wasm_contract() {
    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let contract = runtime::contract_build(&bytes).unwrap();

    let mut store = MemContractStore::new();
    let mut env = MemoryEnv::new(store);

    runtime::contract_store(&contract, &mut env);
}
