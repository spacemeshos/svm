extern crate clap;
extern crate wabt;

use std::fs::File;
use std::io::Read;

use svm_common::Address;
use svm_contract::wasm::Contract;
use svm_runtime;
use wabt::wat2wasm;
use svm_contract::build::WireContractBuilder;

use clap::{Arg, App, SubCommand};

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
    let matches = App::new("svm_cli")
                          .version("0.1")
                          .about("Deploy and call contracts to SVM")
                          .subcommand(SubCommand::with_name("deploy")
                                      .about("Deploy contract")
                                      .arg(Arg::with_name("name")
                                           .short("n")
                                           .long("name")
                                           .value_name("name")
                                           .required(true)
                                           .help("Nome of the contrat to deploy")
                                           .takes_value(true))
                                      .arg(Arg::with_name("address")
                                           .short("a")
                                           .long("address")
                                           .required(true)
                                           .value_name("address")
                                           .help("owner address used to deploy contract")
                                           .takes_value(true)
                                        )
                                      .arg(Arg::with_name("contract")
                                           .short("c")
                                           .long("contract")
                                           .value_name("contract")
                                           .required(true)
                                           .help("wast file of the contract")
                                           .takes_value(true)))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("deploy") {
        let name = matches.value_of("name").unwrap();
        let author_address: u32 = u32::from_str_radix(matches.value_of("address").unwrap()
                                                        .trim_start_matches("0x"), 16).unwrap();
        let file_path = matches.value_of("contract").unwrap();
        println!("Using ADDRESS: {:X}", author_address);
        let (addr, _contract) = deploy_contract(file_path, name, author_address);
    
        println!("Contract deployed to address: {:?}", addr);
    }
    
    
}
