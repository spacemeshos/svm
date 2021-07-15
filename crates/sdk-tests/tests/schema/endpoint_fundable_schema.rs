use serde_json::{json, Value};

use svm_sdk::template;

#[template]
mod Template {
    #[fundable]
    #[endpoint]
    fn call() {}

    #[fundable_hook(default)]
    fn fund() {
        //
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
                "is_fundable": true,
                "doc": "",
                "signature": json!({"params": [], "returns": {}}),
            })],
        })
    );
}
