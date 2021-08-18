use super::wasm_buf_apply;
use crate::api::{self, json::JsonError};

/// Encodes an `Call Account` JSON into SVM binary format.
/// The JSON input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_call(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json| api::json::encode_call_raw(&json.to_string()))
}

/// Decodes a `Call Account` transaction into a JSON,
/// stores that JSON content into a new Wasm Buffer,
/// and finally returns that Wasm buffer offset
pub fn decode_call(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &str| {
        let json = api::json::decode_call(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::json::serde_types::HexBlob;
    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use serde_json::{json, Value};

    #[test]
    fn wasm_call_valid() {
        let target = "1122334455667788990011223344556677889900";

        let verifydata = api::json::encode_inputdata(
            &json!({
                "abi": ["bool", "i8"],
                "data": [true, 3]
            })
            .to_string(),
        )
        .unwrap();

        let calldata = api::json::encode_inputdata(
            &json!({
                "abi": ["i32", "i64"],
                "data": [10, 20]
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
          "version": 1,
          "target": target,
          "func_name": "do_something",
          "verifydata": verifydata["data"],
          "calldata": calldata["data"]
        });

        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_call(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let data = HexBlob(&data[1..]);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        free(json_buf);
        let json_buf = to_wasm_buffer(json.as_bytes());

        free(tx_buf);
        let tx_buf = decode_call(json_buf).unwrap();
        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "target": target,
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
    fn wasm_call_invalid() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_call(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }
}
