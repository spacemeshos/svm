use svm_storage::memory::MemMerklePageCache;
use svm_wasmer::*;

use svm_contract::{
    default::DefaultContractAddressCompute, memory::MemCodeHashStore, types::ContractTypes,
};

struct TestContractTypes;

impl ContractTypes for TestContractTypes {
    type Store = MemCodeHashStore;

    type AddressCompute = DefaultContractAddressCompute;
}

include_svm_runtime!(MemMerklePageCache, TestContractTypes);

// #[test]
// fn deploying_new_contract() {}
