use serde_json::{json, Value};

use svm_sdk::template;

#[template]
mod Template {
    #[ctor(doc = "Initializing a new Account")]
    fn initialize() {}
}

fn main() {
    let raw = raw_meta();
    let json: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(
        json,
        json!({
            "schema": [],
            "api": [json!({
                "name": "initialize",
                "wasm_name": "initialize",
                "is_ctor": true,
                "is_fundable": false,
                "doc": "Initializing a new Account",
                "signature": json!({"params": [], "returns": {}}),
            })],
        })
    );
}
