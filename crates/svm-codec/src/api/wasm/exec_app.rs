use serde_json::Value;

use super::wasm_buf_apply;
use crate::{
    api,
    api::json::{self, JsonError},
};

///
/// Encodes a `exec-app` json input into SVM `exec-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_exec_app(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, api::json::encode_exec_app)
}

pub fn decode_exec_app(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, |json: &Value| {
        let json = api::json::decode_exec_app(json)?;

        api::json::to_bytes(&json)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nibble::NibbleIter;

    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use svm_types::{Address, AppTransaction, WasmValue};

    use serde_json::{json, Value};

    #[test]
    fn wasm_encode_exec_app_valid() {
        let app_addr = "1122334455667788990011223344556677889900";

        let calldata = api::json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20]
        }))
        .unwrap();

        let json = json!({
          "version": 1,
          "app": app_addr,
          "func_index": 2,
          "func_buf": calldata["func_buf"],
          "func_args": calldata["func_args"]
        });

        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_exec_app(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        free(json_buf);
        free(tx_buf);

        let data = json::bytes_to_str(&data[1..]);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = decode_exec_app(json_buf).unwrap();
        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        free(json_buf);
        free(tx_buf);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "app": app_addr,
                "func_index": 2,
                "func_buf": [],
                "func_args": ["10i32", "20i64"],
            })
        );
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
