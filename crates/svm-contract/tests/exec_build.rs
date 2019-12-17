use svm_common::Address;

use svm_contract::wasm::WasmArgValue;
use svm_contract::{
    build::WireTxBuilder,
    env::ContractEnv,
    memory::{MemContractStore, MemoryEnv},
    transaction::Transaction,
};

#[test]
fn build_transaction() {
    let bytes = WireTxBuilder::new()
        .with_version(0)
        .with_contract(Address::from(0x10_20_30_40))
        .with_sender(Address::from(0x50_60_70_80))
        .with_func_name("run")
        .with_func_args(&vec![WasmArgValue::I32(10), WasmArgValue::I64(20)])
        .build();

    let store = MemContractStore::new();
    let env = MemoryEnv::new(store);

    let actual = env.build_transaction(&bytes).unwrap();

    let expected = Transaction {
        contract: Address::from(0x10_20_30_40),
        sender: Address::from(0x50_60_70_80),
        func_name: "run".to_string(),
        func_args: vec![WasmArgValue::I32(10), WasmArgValue::I64(20)],
    };

    assert_eq!(expected, actual);
}
