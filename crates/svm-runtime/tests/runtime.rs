use svm_common::{Address, State};
use svm_contract::build::{WireContractBuilder, WireTxBuilder};
use svm_contract::wasm::WasmArgValue as Value;

use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};
use svm_storage::PageSliceCache;

// Injects `svm` runtime backed by `rocksdb` into the current file.
svm_runtime::include_svm_rocksdb_runtime!("tests-contract-storage", "tests-contract-code");

macro_rules! build_raw_contract {
    ($version: expr, $name: expr, $author: expr, $file: expr) => {{
        let wasm = load_wasm_file!($file);

        WireContractBuilder::new()
            .with_version($version)
            .with_name($name)
            .with_author(Address::from($author))
            .with_code(&wasm[..])
            .build()
    }};
}

macro_rules! build_raw_tx {
    ($version: expr, $contract_addr: expr, $sender_addr: expr, $func_name: expr, $func_args: expr) => {{
        WireTxBuilder::new()
            .with_version($version)
            .with_contract($contract_addr)
            .with_sender(Address::from($sender_addr))
            .with_func_name($func_name)
            .with_func_args($func_args)
            .build()
    }};
}

macro_rules! exec_tx {
    ($tx: expr, $state: expr) => {{
        let opts = svm_runtime::opts::Opts { max_pages: 10 };

        let import_object =
            runtime::import_object_create($tx.contract.clone(), $state, std::ptr::null(), opts);

        runtime::contract_exec($tx, &import_object)
    }};
}

macro_rules! load_wasm_file {
    ($file: expr) => {{
        let wasm = include_str!($file);
        wabt::wat2wasm(&wasm).unwrap()
    }};
}

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
    let addr = runtime::contract_compute_address(&contract);
    runtime::contract_store(&contract, &addr);
}

#[test]
#[ignore]
fn contract_exec_non_existing_contract() {
    // ...
}

#[test]
fn contract_exec_valid_transaction() {
    // 1) deploying the contract
    let bytes = build_raw_contract!(
        0,                     // protocol version
        "Contract #1",         // contract name
        0x10_20_30_40,         // author address
        "wasm/runtime-1.wast"  // file holding the wasm code
    );
    let contract = runtime::contract_build(&bytes).unwrap();
    let addr = runtime::contract_compute_address(&contract);
    runtime::contract_store(&contract, &addr);

    // 2) executing a transaction `reg_set_and_persist`
    // setting register `64:0` the value `1000`.
    // then, persisting it to storage (page=`0`, slice=`0`, offset=`0`)
    let bytes = build_raw_tx!(
        0,                     // protocol version
        addr.clone(),          // contract address
        0x11_22_33_44,         // sender address
        "reg_set_and_persist", // `func_name` to execute
        // `func_args`
        &[
            Value::I64(0x10_20_30_40_50_60_70_80),
            Value::I32(64),
            Value::I32(0),
            Value::I32(0),
            Value::I32(0),
            Value::I32(0)
        ]
    );

    let tx = runtime::transaction_build(&bytes).unwrap();

    let receipt = exec_tx!(tx, State::from(0));
    assert_eq!(true, receipt.success);
    assert_eq!(None, receipt.error);

    let new_state = receipt.new_state.unwrap();
    assert_ne!(State::from(0), new_state);

    let pages_storage =
        svm_runtime::gen_rocksdb_pages_storage!(addr, new_state, 10, "tests-contract-storage");
    let page_cache = svm_runtime::gen_rocksdb_page_cache!(pages_storage, 10);
    let mut storage = PageSliceCache::new(page_cache);

    let slice_pos = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);

    let slice = storage.read_page_slice(&slice_pos).unwrap();
    assert_eq!(
        &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
        &slice[..]
    );
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
