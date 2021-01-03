#![allow(unused)]
use serde_json::{json, Value};
use svm_sdk::{app, Address, Amount};

#[app]
mod App {
    #[ctor]
    fn init() -> (bool, u64, Amount, Address) {}
}

fn main() {
    let raw = raw_schema();

    let json: Value = serde_json::from_str(&raw).unwrap();

    dbg!(json);

    // assert_eq!(
    //     json,
    //     json!({
    //         "storage": [],
    //         "exports": [json!({
    //             "api_name": "init",
    //             "wasm_name": "init",
    //             "is_ctor": true,
    //             "is_fundable": false,
    //             "signature": json!({"params": [], "returns": []}),
    //         })],
    //     })
    // );
}
