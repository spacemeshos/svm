use serde_json::{json, Value};
use svm_sdk::app;

#[app]
mod App {
    #[ctor]
    fn init() {}
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
                "signature": json!({"params": [], "returns": {}}),
            })],
        })
    );
}
