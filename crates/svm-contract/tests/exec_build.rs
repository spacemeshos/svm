use svm_common::Address;

use svm_contract::*;
use svm_contract::{env::ContractEnv, memory::MemoryEnv, wasm::WasmArgValue};

#[test]
fn exec_contract() {
    let bytes = WireTxBuilder::new()
        .with_version(0)
        .with_contract(Address::from(0x10_20_30_40))
        .with_sender(Address::from(0x50_60_70_80))
        .with_func_name("run")
        .with_func_args(&vec![WasmArgValue::I32(10), WasmArgValue::I64(20)])
        .build();

    let _tx = <MemoryEnv as ContractEnv>::exec_contract(&bytes).unwrap();
}
