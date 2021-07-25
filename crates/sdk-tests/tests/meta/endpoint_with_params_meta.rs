#![allow(unused)]
use serde_json::{json, Value};

use svm_sdk::{template, Address, Amount};

#[template]
mod Template {
    #[endpoint]
    fn call(a: bool, b: u64, c: Amount, d: Address) {}
}

fn main() {
    let raw = raw_meta();
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
                "signature": json!({"params": [
                    json!({"name": "a", "type": "bool"}),
                    json!({"name": "b", "type": "u64"}),
                    json!({"name": "c", "type": "Amount"}),
                    json!({"name": "d", "type": "Address"}),
                ], "returns": {}}),
            })],
        })
    );
}
