use serde_json::Value;

use super::wasm_buf_apply;
use crate::api::{self, json::JsonError};

/// Encodes an `exec-app` JSON into SVM `exec-app` binary transaction.
/// The JSON input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_exec_app(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, api::json::encode_exec_app)
}

/// Decodes an `exec-app` transaction into a JSON,
/// stores that JSON content into a new Wasm Buffer,
/// and finally returns that Wasm buffer offset
pub fn decode_exec_app(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &Value| {
        let json = api::json::decode_exec_app(json)?;

        api::json::to_bytes(&json)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::json;
    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use serde_json::{json, Value};

    #[test]
    fn wasm_encode_exec_app_valid() {
        let app_addr = "1122334455667788990011223344556677889900";

        let verifydata = api::json::encode_calldata(&json!({
            "abi": ["bool", "i8"],
            "data": [true, 3]
        }))
        .unwrap();

        let calldata = api::json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20]
        }))
        .unwrap();

        let json = json!({
          "version": 1,
          "app": app_addr,
          "func_name": "do_something",
          "verifydata": verifydata["calldata"],
          "calldata": calldata["calldata"]
        });

        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_exec_app(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let data = json::bytes_to_str(&data[1..]);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        free(json_buf);
        let json_buf = to_wasm_buffer(json.as_bytes());

        free(tx_buf);
        let tx_buf = decode_exec_app(json_buf).unwrap();
        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "app": app_addr,
                "func_name": "do_something",
                "verifydata": {
                    "abi": ["bool", "i8"],
                    "data": [true, 3],
                },
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20],
                }
            })
        );

        free(json_buf);
        free(tx_buf);
    }

    #[test]
    fn wasm_encode_exec_app_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_exec_app(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }
}
