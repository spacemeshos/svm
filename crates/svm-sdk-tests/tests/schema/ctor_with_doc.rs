use serde_json::{json, Value};
use svm_sdk::app;

#[app]
mod App {
    #[ctor(doc = "Initializing a new app")]
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
                "doc": "Initializing a new app",
                "signature": json!({"params": [], "returns": {}}),
            })],
        })
    );
}
