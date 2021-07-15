#![allow(unused)]
use serde_json::{json, Value};
use svm_sdk::{template, Address, Amount};

#[template]
mod Template {
    #[endpoint]
    fn call() -> Amount {
        Amount(20)
    }
}

fn main() {
    let raw = raw_schema();
    let json: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(
        json,
        json!({
           "storage": [],
           "exports": [json!({
                "api_name": "call",
                "wasm_name": "call",
                "is_ctor": false,
                "is_fundable": false,
                "doc": "",
                "signature": json!({
                    "params": [],
                    "returns": { "type": "Amount" },
                }),
            })],
        })
    );
}
