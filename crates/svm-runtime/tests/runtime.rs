use svm_common::{Address, State};
use svm_contract::build::{WireContractBuilder, WireTxBuilder};
use svm_contract::wasm::{WasmArgType, WasmArgValue as Value};
use svm_runtime::*;

use svm_storage::page::{PageIndex, PageSliceLayout, SliceIndex};
use svm_storage::PageSliceCache;

macro_rules! gen_rocksdb_pages_storage {
    ($addr: expr, $state: expr, $max_pages: expr, $contract_storage_path: expr) => {{
        use std::cell::RefCell;
        use std::path::Path;
        use std::rc::Rc;

        use svm_kv::rocksdb::RocksStore;
        use svm_storage::rocksdb::RocksPages;

        let path = Path::new($contract_storage_path);
        let kv = RocksStore::new(path);
        let kv = Rc::new(RefCell::new(kv));

        RocksPages::new($addr, kv, $state, $max_pages as u32)
    }};
}

macro_rules! gen_rocksdb_page_cache {
    ($pages_storage: expr, $max_pages: expr) => {{
        use svm_storage::rocksdb::RocksMerklePageCache;

        RocksMerklePageCache::new($pages_storage, $max_pages)
    }};
}

macro_rules! gen_rocksdb_env {
    ($code_db_path: expr) => {{
        use std::path::Path;
        use svm_contract::rocksdb::{RocksContractStore, RocksEnv};

        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let path = Path::new($code_db_path);
        let store = RocksContractStore::<S, D>::new(path);

        RocksEnv::new(store)
    }};
}

include_svm_runtime!(
    |addr, state, max_pages| {
        gen_rocksdb_pages_storage!(addr, state, max_pages, "tests-contract-storage")
    },
    |pages_storage, max_pages| gen_rocksdb_page_cache!(pages_storage, max_pages),
    svm_storage::rocksdb::RocksMerklePageCache,
    svm_contract::rocksdb::RocksEnv,
    || gen_rocksdb_env!("tests-contract-code")
);

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
        let opts = svm_runtime::opts::Opts {
            max_pages: 10,
            max_pages_slices: 100,
        };

        let import_object =
            runtime::import_object_create($tx.contract, $state, std::ptr::null(), opts);

        runtime::contract_exec(&$tx, &import_object)
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
    runtime::contract_store(&contract);
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
    runtime::contract_store(&contract);

    let contract_addr = contract.address.unwrap();

    // 2) executing a transaction `reg_set_and_persist`
    // setting register `64:0` the value `1000`.
    // then, persisting it to storage (page=`0`, slice=`0`, offset=`0`)
    let bytes = build_raw_tx!(
        0,                     // protocol version
        contract_addr,         // contract address
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

    let new_state = exec_tx!(tx, State::from(0)).unwrap();
    assert_ne!(State::from(0), new_state);

    // reading data from storage
    let mut pages_storage =
        gen_rocksdb_pages_storage!(contract_addr, new_state, 10, "tests-contract-storage");
    let page_cache = gen_rocksdb_page_cache!(&mut pages_storage, 10);

    let boxed_pc = Box::new(page_cache);
    let pc = Box::leak(boxed_pc);

    let storage = PageSliceCache::new(pc, 100);
    let boxed_storage = Box::new(storage);
    let storage = Box::leak(boxed_storage);

    let slice_pos = PageSliceLayout {
        slice_idx: SliceIndex(0),
        page_idx: PageIndex(0),
        offset: 0,
        len: 8,
    };

    let slice = storage.read_page_slice(&slice_pos).unwrap();
    assert_eq!(
        &[0x80, 0x70, 0x60, 0x50, 0x40, 0x30, 0x20, 0x10],
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
