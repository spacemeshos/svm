use serde_json::{json, Value};
use svm_sdk::app;

#[app]
mod App {
    #[endpoint(doc = "ANDing `a` and `b`")]
    fn and(a: bool, b: bool) -> bool {
        a && b
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
                "api_name": "and",
                "wasm_name": "and",
                "is_ctor": false,
                "is_fundable": false,
                "doc": "ANDing `a` and `b`",
                "signature": json!({"params": [
                    json!({"name": "a", "type": "bool"}),
                    json!({"name": "b", "type": "bool"}),
                ], "returns": {"type": "bool"}}),
            })],
        })
    );
}
