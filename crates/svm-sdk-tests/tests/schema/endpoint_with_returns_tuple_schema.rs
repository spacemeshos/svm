#![allow(unused)]
use serde_json::{json, Value};
use svm_sdk::{app, Address, Amount};

#[app]
mod App {
    #[endpoint]
    fn call() -> (bool, u64, Amount) {
        (true, 10, Amount(20))
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
                "signature": json!({
                    "params": [],
                    "returns": [
                        { "type": "bool" },
                        { "type": "u64" },
                        { "type": "Amount" },
                     ]
                }),
            })],
        })
    );
}
