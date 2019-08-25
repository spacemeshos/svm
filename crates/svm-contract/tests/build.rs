use svm_common::Address;
use svm_contract::*;
use svm_contract::{
    default::DefaultContractAddressCompute, env::ContractEnv, memory::MemContractStore,
    traits::ContractAddressCompute,
};

struct TestEnv {}

impl ContractEnv for TestEnv {
    type Store = MemContractStore;

    type AddressCompute = DefaultContractAddressCompute;

    fn init_store<F: Fn() -> Self::Store>(&mut self, _f: F) {
        unimplemented!()
    }

    fn get_store(&mut self) -> &Self::Store {
        unimplemented!()
    }

    fn get_store_mut(&mut self) -> &mut Self::Store {
        unimplemented!()
    }

    fn open_store(&mut self) {
        unimplemented!()
    }

    fn close_store(&mut self) {
        unimplemented!()
    }
}

#[test]
fn build_wasm_contract() {
    let bytes = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
        .build();

    let contract = svm_contract::build_wasm_contract::<TestEnv>(&bytes).unwrap();

    let expected_addr = DefaultContractAddressCompute::compute(&contract);
    let actual_addr = contract.address.as_ref().unwrap();
    assert_eq!(expected_addr.as_slice(), actual_addr.as_slice());

    assert_eq!("Contract #1", contract.name);
    assert_eq!(Address::from(0x10_20_30_40), contract.author);
    assert_eq!([0xAA, 0xBB, 0xCC, 0xDD], contract.wasm.as_ref());
}
