use serde_json::{json, Value};

use svm_sdk::template;

#[template]
mod Template {
    #[ctor(doc = "Initializing a new Account")]
    fn initialize() {}
}

fn main() {
    let raw = raw_schema();
    let json: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(
        json,
        json!({
            "storage": [],
            "exports": [json!({
                "api_name": "initialize",
                "wasm_name": "initialize",
                "is_ctor": true,
                "is_fundable": false,
                "doc": "Initializing a new Account",
                "signature": json!({"params": [], "returns": {}}),
            })],
        })
    );
}
