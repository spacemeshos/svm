extern crate clap;
extern crate wabt;

use std::fs::File;
use std::io::Read;

use svm_common::{Address, State};
use svm_runtime;
use svm_runtime::runtime::Receipt;
use wabt::wat2wasm;
use svm_contract::wasm::Contract;
use svm_contract::wasm::WasmArgValue as Value;
use svm_contract::build::{WireContractBuilder, WireTxBuilder};
use svm_contract::transaction::Transaction;
use svm_contract::error::TransactionBuildError;

use clap::{Arg, App, SubCommand};

svm_runtime::include_svm_rocksdb_runtime!("tests-contract-storage", "tests-contract-code");
// Exec transaction

fn build_tx(version: u32, contract_addr: Address, sender_addr: Address, 
    func_name: String, func_args: Vec<Value>) -> Vec<u8> {
        return WireTxBuilder::new()
            .with_version(version)
            .with_contract(contract_addr)
            .with_sender(Address::from(sender_addr))
            .with_func_name(&func_name)
            .with_func_args(&func_args)
            .build()
}


fn exec_tx (tx: Transaction, state: State) -> Receipt {
        let opts = svm_runtime::opts::Opts {
            max_pages: 10,
            max_pages_slices: 100,
        };

        let import_object =
            runtime::import_object_create(tx.contract.clone(), state, std::ptr::null(), opts);

        runtime::contract_exec(tx, &import_object)
}

fn build_args(raw_args: Vec<&str>) -> Vec<Value> {
    return vec![
        Value::I64(0x10_20_30_40_50_60_70_80),
        Value::I32(64),
        Value::I32(0),
        Value::I32(0),
        Value::I32(0),
        Value::I32(0)
    ]
}

fn execute_transaction(version: u32, contract_addr: u32, sender_addr: u32, 
    func_name: String, func_args: Vec<&str>) -> Result<Receipt, TransactionBuildError> {
        let args = build_args(func_args);
        let bytes = build_tx(
            0,                     
            Address::from(contract_addr), 
            Address::from(sender_addr),
            func_name,
            args
        );
    
        return match runtime::transaction_build(&bytes) {
            Ok(tx) => { Ok(exec_tx(tx, State::from(0)))}
            Err(e) => { Err(e) }
        };
}


// Deploy contract functionality
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
                          .subcommand(SubCommand::with_name("tx")
                                           .about("Execute transaction")
                                           .arg(Arg::with_name("fn_name")
                                                .short("fn")
                                                .long("name")
                                                .value_name("fn_name")
                                                .required(true)
                                                .help("Nome of the contrat to deploy")
                                                .takes_value(true))
                                           .arg(Arg::with_name("sender_address")
                                                .short("a")
                                                .long("address")
                                                .required(true)
                                                .value_name("sender_address")
                                                .help("owner address used to deploy contract")
                                                .takes_value(true))
                                           .arg(Arg::with_name("contract_address")
                                                .short("c")
                                                .long("contract")
                                                .value_name("contract_address")
                                                .required(true)
                                                .takes_value(true))
                                            .arg(Arg::with_name("fn_args").multiple(true).last(true))
                                            )
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

    if let Some(matches) = matches.subcommand_matches("tx") {
        let name = matches.value_of("fn_name").unwrap();
        let args = matches.values_of("fn_args").map(|vals| vals.collect::<Vec<_>>()).unwrap();
        let sender_address: u32 = u32::from_str_radix(matches.value_of("sender_address").unwrap()
                                                        .trim_start_matches("0x"), 16).unwrap();
        let contract_address: u32 = u32::from_str_radix(matches.value_of("contract_address").unwrap()
                                                        .trim_start_matches("0x"), 16).unwrap();
        
        let res = execute_transaction(
            0,                     // protocol version
            contract_address,          // contract address
            sender_address,         // sender address
            name.to_string(),
            args
        );
    
        println!("Tx executed: {:?}", res);
    }
}
