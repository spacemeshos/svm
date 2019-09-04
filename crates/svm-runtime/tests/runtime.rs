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

        let path = Path::new("leveldb");
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

        let path = Path::new("leveldb");
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

#[test]
fn contract_exec_valid_transaction() {
    let wasm = load_wasm_file!("wasm/runtime-1.wast");

    let raw_contract = WireContractBuilder::new()
        .with_version(0)
        .with_name("Contract #1")
        .with_author(Address::from(0x10_20_30_40))
        .with_code(&wasm[..])
        .build();

    let contract = runtime::contract_build(&raw_contract).unwrap();
    runtime::contract_store(&contract);

    let contract_addr = contract.address.unwrap();

    let raw_tx = WireTxBuilder::new()
        .with_version(0)
        .with_contract(contract_addr)
        .with_sender(Address::from(0x11_22_33_44))
        .with_func_name("do_reg_set")
        .with_func_args(&[Value::I64(1000), Value::I32(64), Value::I32(0)])
        .build();

    let tx = runtime::transaction_build(&raw_tx).unwrap();

    let opts = svm_runtime::opts::Opts {
        max_pages: 10,
        max_pages_slices: 100,
    };

    let import_object =
        runtime::import_object_create(contract_addr, State::from(0), std::ptr::null(), opts);

    let res = runtime::contract_exec(&tx, &import_object);
    dbg!(res);
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
