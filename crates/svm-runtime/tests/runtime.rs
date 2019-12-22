use svm_common::State;
use svm_contract::wasm::WasmArgValue as Value;
use svm_runtime::{contract_settings::ContractSettings, testing, traits::Runtime};
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
//     let addr = runtime::contract_derive_address(&contract);
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
    let host = std::ptr::null();
    let exts = Vec::new();
    let mut runtime = testing::create_memory_runtime(host, &kv, exts);
    let contract = runtime.contract_build(&bytes).unwrap();
    let addr = runtime.contract_derive_address(&contract);

    runtime.contract_deploy(&contract, &addr);

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
        ],
    );

    let tx = runtime.transaction_build(&bytes).unwrap();

    let settings = ContractSettings {
        pages_count: 5,
        kv_path: String::new(),
    };

    let receipt = runtime.transaction_exec(&tx, &State::empty(), &settings);

    assert_eq!(true, receipt.success);
    assert_eq!(None, receipt.error);

    let new_state = receipt.new_state.unwrap();
    assert_ne!(State::from(0), new_state);

    // now we'll read directly from the contract's storage and assert that the
    // data has been persisted as expected.

    let mut storage = runtime.open_contract_storage(&addr, &new_state, &settings);

    let layout = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);
    let slice = storage.read_page_slice(&layout);

    assert_eq!(vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80], slice);
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
