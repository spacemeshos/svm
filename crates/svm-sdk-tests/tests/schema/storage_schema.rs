use serde_json::{json, Value};
use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        a: bool,
        b: u8,
        c: i8,
        d: u16,
        e: i16,
        f: u32,
        g: i32,
        h: u64,
        i: i64,
        j: Amount,
        k: Address,
    }
}

fn main() {
    let raw = raw_schema();

    let json: Value = serde_json::from_str(&raw).unwrap();

    assert_eq!(
        json,
        json!({
            "storage": [
                {"id": 0,  "name": "a", "type": "bool",    "offset": 0,  "byte_count": 1},
                {"id": 1,  "name": "b", "type": "u8",      "offset": 1,  "byte_count": 1},
                {"id": 2,  "name": "c", "type": "i8",      "offset": 2,  "byte_count": 1},
                {"id": 3,  "name": "d", "type": "u16",     "offset": 3,  "byte_count": 2},
                {"id": 4,  "name": "e", "type": "i16",     "offset": 5,  "byte_count": 2},
                {"id": 5,  "name": "f", "type": "u32",     "offset": 7,  "byte_count": 4},
                {"id": 6,  "name": "g", "type": "i32",     "offset": 11, "byte_count": 4},
                {"id": 7,  "name": "h", "type": "u64",     "offset": 15, "byte_count": 8},
                {"id": 8,  "name": "i", "type": "i64",     "offset": 23, "byte_count": 8},
                {"id": 9,  "name": "j", "type": "Amount",  "offset": 31, "byte_count": 8},
                {"id": 10, "name": "k", "type": "Address", "offset": 39, "byte_count": 20},
            ],
            "exports": [],
        })
    );
}
