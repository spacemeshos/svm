use svm_common::Address;
use svm_contract::WireContractBuilder;
use svm_storage::memory::MemMerklePageCache;
use svm_wasmer::*;

use svm_contract::{default::DefaultContractAddressCompute, memory::MemCodeHashStore};

struct TestContractTypes;

// include_svm_runtime!(MemMerklePageCache, TestContractTypes);

// #[test]
// fn deploy_wasm_contract() {
//     let bytes = WireContractBuilder::new()
//         .with_version(0)
//         .with_name("Contract #1")
//         .with_author(Address::from(0x10_20_30_40))
//         .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
//         .build();
//
//     let wasm_contract = contract_build(&bytes).unwrap();
//
//     // contract_store(&contract);
//     unimplemented!()
// }
