use std::ffi::c_void;
use std::rc::Rc;

use svm_common::State;
use svm_contract::wasm::WasmArgValue as Value;
use svm_kv::memory::MemKVStore;
use svm_runtime::{opts::Opts, testing, Runtime};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};

// #[test]
// #[ignore]
// fn deploy_wasm_contract() {
//     let bytes = WireContractBuilder::new()
//         .with_version(0)
//         .with_name("Contract #1")
//         .with_author(Address::from(0x10_20_30_40))
//         .with_code(&[0xAA, 0xBB, 0xCC, 0xDD])
//         .build();
//
//     let contract = runtime::contract_build(&bytes).unwrap();
//     let addr = runtime::contract_compute_address(&contract);
//     runtime::contract_store(&contract, &addr);
// }
//
// #[test]
// #[ignore]
// fn contract_exec_non_existing_contract() {
//     // ...
// }

#[test]
fn runtime_executing_a_valid_transaction() {
    // 1) deploying the contract
    let bytes = testing::build_raw_contract(
        0,
        "Contract #1",
        0x10_20_30_40,
        include_str!("wasm/runtime-1.wast"),
    );

    let kv = testing::memory_kv_store_init();
    let mut runtime = testing::create_memory_runtime(&kv);
    let contract = runtime.contract_build(&bytes).unwrap();
    let addr = runtime.contract_compute_address(&contract);
    runtime.contract_store(&contract, &addr);

    // 2) executing a transaction `reg_set_and_persist`
    // setting register `64:0` the value `1_000`.
    // then, persisting it to contract-storage (page=`0`, offset=`0`)
    let bytes = testing::build_raw_transaction(
        0,                     // protocol version
        &addr,                 // contract address
        0x11_22_33_44,         // sender address
        "reg_set_and_persist", // `func_name` to execute
        // `func_args`
        &[
            Value::I64(0x10_20_30_40_50_60_70_80),
            Value::I32(64),
            Value::I32(0),
            Value::I32(0),
            Value::I32(0),
            Value::I32(0),
        ],
    );

    let tx = runtime.transaction_build(&bytes).unwrap();

    let node_data: *const c_void = std::ptr::null() as _;
    let opts = Opts {
        max_pages: 5,
        kv_path: String::new(),
    };

    let import_object = runtime.import_object_create(addr, State::empty(), node_data, &opts);
    let receipt = runtime.contract_exec(tx, &import_object);
    dbg!(receipt);

    // assert_eq!(true, receipt.success);
    // assert_eq!(None, receipt.error);

    // let new_state = receipt.new_state.unwrap();
    // assert_ne!(State::from(0), new_state);
    //
    // let pages_storage =
    //     svm_runtime::gen_rocksdb_pages_storage!(addr, new_state, 10, "tests-contract-storage");
    // let page_cache = svm_runtime::gen_rocksdb_page_cache!(pages_storage, 10);
    // let mut storage = ContractStorage::new(Box::new(page_cache));
    //
    // let slice_pos = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);
    //
    // let slice = storage.read_page_slice(&slice_pos);
    // assert_eq!(
    //     &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
    //     &slice[..]
    // );
}

// #[test]
// #[ignore]
// fn contract_exec_invalid_state() {
//     //
// }
//
// #[test]
// #[ignore]
// fn contract_exec_invalid_func_name() {
//     //
// }
//
// #[test]
// #[ignore]
// fn contract_exec_invalid_func_args() {
//     //
// }
