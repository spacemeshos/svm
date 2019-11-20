extern crate wabt;

use std::fs::File;
use std::io::Read;

use svm_common::Address;
use svm_contract::wasm::Contract;
use svm_runtime;
use wabt::wat2wasm;
use svm_contract::build::WireContractBuilder;

fn build_contract(version: u32, name: &str, author_address: u32, wasm: &[u8]) -> Vec<u8> {
    return WireContractBuilder::new()
        .with_version(version)
        .with_name(name)
        .with_author(Address::from(author_address))
        .with_code(&wasm[..])
        .build();
}

fn get_file_content(file_path: &str) -> String {
    let mut content = String::new();
    let mut file_handler = File::open(file_path).expect("Unable to open file");
    file_handler.read_to_string(&mut content).expect("Unable to read file content");
    return content;
}

fn string_to_wasm(content: &str) -> Vec<u8> {
    return wat2wasm(&content).expect("Unable to convert wast to wasm");
}

fn deploy_contract(file_path: &str, name: &str, author_address: u32) ->  (Address, Contract) {
    let version = 0;
    let content = get_file_content(file_path);
    let wasm = string_to_wasm(&content);
    let raw_contract = build_contract(version, name, author_address, &wasm);
    svm_runtime::include_svm_rocksdb_runtime!("tests-contract-storage", "tests-contract-code");

    let contract = runtime::contract_build(&raw_contract).expect("Cannot build the contract");
    let addr = runtime::contract_compute_address(&contract);
    runtime::contract_store(&contract, &addr);

    return (addr, contract);
}

fn main() {
    let name = "Contract #1";
    let author_address = 0x10_20_30_40;
    let file_path = "crates/svm-runtime/tests/wasm/runtime-1.wast";
    let (addr, _contract) = deploy_contract(file_path, name, author_address);

    println!("Hello, world! {:?}", addr);
}
