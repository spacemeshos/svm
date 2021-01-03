#![allow(unused)]
use serde_json::{json, Value};
use svm_sdk::{app, Address, Amount};

#[app]
mod App {
    #[ctor]
    fn init(a: bool, b: u64, c: Amount, d: Address) {}
}

fn main() {
    let raw = raw_schema();

    let json: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(
        json,
        json!({
            "storage": [],
            "exports": [json!({
                "api_name": "init",
                "wasm_name": "init",
                "is_ctor": true,
                "is_fundable": false,
                "signature": json!({"params": [
                    json!({"name": "a", "type": "bool"}),
                    json!({"name": "b", "type": "u64"}),
                    json!({"name": "c", "type": "Amount"}),
                    json!({"name": "d", "type": "Address"}),
                ], "returns": []}),
            })],
        })
    );
}
