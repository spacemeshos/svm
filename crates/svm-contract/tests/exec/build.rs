use crate::traits::ContractAddressCompute;
use svm_common::Address;
use svm_contract::default::DefaultContractAddressCompute;
use svm_contract::env::ContractEnv;
use svm_contract::memory::MemoryEnv;

use svm_contract::*;

#[test]
fn build_wasm_contract() {
    let bytes = WireTxBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let contract = <MemoryEnv as ContractEnv>::build_contract(&bytes).unwrap();

    let expected_addr = DefaultContractAddressCompute::compute(&contract);
    let actual_addr = contract.address.as_ref().unwrap();
    assert_eq!(expected_addr.as_slice(), actual_addr.as_slice());

    assert_eq!("Contract #1", contract.name);
    assert_eq!(Address::from(0x10_20_30_40), contract.author);
    assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], contract.wasm.as_ref());
}
