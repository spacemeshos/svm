use serde_json::{json, Value};
use svm_sdk::{app, Amount};

#[app]
mod App {
    #[fundable(default_funding)]
    #[endpoint]
    fn call() {}

    #[fundable_hook]
    fn default_funding(_value: Amount) {
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
                "signature": json!({"params": [], "returns": []}),
            })],
        })
    );
}
