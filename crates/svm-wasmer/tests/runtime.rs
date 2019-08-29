use svm_common::Address;
use svm_contract::build::WireContractBuilder;
use svm_wasmer::*;

include_svm_runtime!(
    svm_storage::memory::MemMerklePageCache,
    svm_contract::memory::MemoryEnv,
    || {
        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let store = svm_contract::memory::MemContractStore::<S, D>::new();

        svm_contract::memory::MemoryEnv::new(store)
    }
);

#[test]
#[ignore]
fn deploy_wasm_contract() {
    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let contract = runtime::contract_build(&bytes).unwrap();
    runtime::contract_store(&contract);
}

#[test]
#[ignore]
fn contract_exec_non_existing_contract() {
    //
}

#[test]
#[ignore]
fn contract_exec_valid_transaction() {
    //
}

#[test]
#[ignore]
fn contract_exec_invalid_state() {
    //
}

#[test]
#[ignore]
fn contract_exec_invalid_func_name() {
    //
}

#[test]
#[ignore]
fn contract_exec_invalid_func_args() {
    //
}
