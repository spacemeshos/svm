#![allow(unused)]

use serde_json::{json, Value};

use svm_sdk_mock::{template, Address, Amount};

#[template]
mod Template {
    #[endpoint]
    fn call(a: [bool; 3]) {}
}

fn main() {
    let raw = raw_meta();
    let json: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(
        json,
        json!({
            "schema": [],
            "api": [json!({
                "name": "call",
                "wasm_name": "call",
                "is_ctor": false,
                "is_fundable": false,
                "doc": "",
                "signature": json!({"params": [
                    json!({"name": "a", "type": "[bool]", "length": 3}),
                ], "returns": {}}),
            })],
        })
    );
}
