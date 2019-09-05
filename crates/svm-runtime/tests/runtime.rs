use svm_common::{Address, State};
use svm_contract::build::{WireContractBuilder, WireTxBuilder};
use svm_contract::wasm::{WasmArgType, WasmArgValue as Value};
use svm_runtime::*;

include_svm_runtime!(
    |addr, state, max_pages| {
        use std::cell::RefCell;
        use std::path::Path;
        use std::sync::Arc;

        use svm_kv::leveldb::LDBStore;
        use svm_storage::leveldb::LDBPages;

        let path = Path::new("ldb-contract-storage");
        let kv = LDBStore::new(path);
        let kv = Arc::new(RefCell::new(kv));

        LDBPages::new(addr, Arc::clone(&kv), state, max_pages as u32)
    },
    |arg_pages_storage, arg_max_pages| {
        use svm_storage::leveldb::LDBMerklePageCache;

        LDBMerklePageCache::new(arg_pages_storage, arg_max_pages)
    },
    svm_storage::leveldb::LDBMerklePageCache,
    svm_contract::leveldb::LDBEnv,
    || {
        use std::path::Path;
        use svm_contract::leveldb::{LDBContractStore, LDBEnv};

        use svm_contract::wasm::{
            WasmContractJsonDeserializer as D, WasmContractJsonSerializer as S,
        };

        let path = Path::new("ldb-contract-code");
        let store = LDBContractStore::<S, D>::new(path);

        LDBEnv::new(store)
    }
);

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

    // 2) executing a transaction `do_reg_set`. setting register `64:0` the value `1000`.
    let bytes = build_raw_tx!(
        0,                                                  // protocol version
        contract_addr,                                      // contract address
        0x11_22_33_44,                                      // sender address
        "do_reg_set",                                       // `func_name` to execute
        &[Value::I64(1000), Value::I32(64), Value::I32(0)]  // `func_args`
    );

    let tx = runtime::transaction_build(&bytes).unwrap();

    let new_state = exec_tx!(tx, State::from(0)).unwrap();
    assert_ne!(State::from(0), new_state);

    // release memory
    // ??????

    // let bytes = build_raw_tx!(
    //     0,                   // protocol version
    //     contract_addr,       // contract address
    //     0x11_22_33_44,       // sender address
    //     "do_write_from_reg", // `func_name` to execute
    //     // `func_args`
    //     &[
    //         Value::I32(64),
    //         Value::I32(0),
    //         Value::I32(4),
    //         Value::I32(0),
    //         Value::I32(0),
    //         Value::I32(0)
    //     ]
    // );
    //
    // let tx = runtime::transaction_build(&bytes).unwrap();
    //
    // // executing the 2nd transaction.
    // let new_state = exec_tx!(tx, new_state).unwrap();
    // dbg!(new_state);
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
