use svm_common::Address;

use svm_contract::wasm::WasmArgValue;
use svm_contract::{Tx, WireTxBuilder};

#[test]
fn exec_contract_tx_parse() {
    let bytes = WireTxBuilder::new()
        .with_version(0)
        .with_contract(Address::from(0x10_20_30_40))
        .with_sender(Address::from(0x50_60_70_80))
        .with_func_name("run")
        .with_func_args(&vec![WasmArgValue::I32(10), WasmArgValue::I64(20)])
        .build();

    let actual = svm_contract::parse_tx(&bytes).unwrap();

    let expected = Tx {
        contract: Address::from(0x10_20_30_40),
        sender: Address::from(0x50_60_70_80),
        func_name: "run".to_string(),
        func_args: vec![WasmArgValue::I32(10), WasmArgValue::I64(20)],
    };

    assert_eq!(expected, actual);
}
