use super::wasm_buf_apply;
use crate::api::{self, json::JsonError};

/// Encodes a `Spawn Account` JSON input into SVM binary format.
/// The JSON input is passed by giving WASM memory start address (`offset` parameter).
///
/// Returns an offset to a Wasm buffer holding the encoded transaction (wrapped within a JSON)
pub fn encode_spawn(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, api::json::encode_spawn)
}

/// Decodes a binary `Spawn Account` transaction given as a Wasm buffer (the `offset` parameter),
///
/// and returns a new Wasm buffer holding the decoded transaction (wrapped with a JSON).
pub fn decode_spawn(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &str| {
        let json = api::json::decode_spawn(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::json;
    use crate::api::json::wrappers::HexBlob;
    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use serde_json::{json, Value};

    #[test]
    fn wasm_spawn_valid() {
        let template_addr = "1122334455667788990011223344556677889900";

        let calldata = json::encode_calldata(
            &json!({
                "abi": ["i32", "i64"],
                "data": [10, 20]
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
          "version": 1,
          "template": template_addr,
          "name": "My Account",
          "ctor_name": "initialize",
          "calldata": calldata["calldata"],
        });

        let json = serde_json::to_string(&json).unwrap();
        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_spawn(json_buf).unwrap();
        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let data = HexBlob(&data[1..]);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        free(json_buf);
        let json_buf = to_wasm_buffer(json.as_bytes());

        free(tx_buf);
        let tx_buf = decode_spawn(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "template": template_addr,
                "name": "My Account",
                "ctor_name": "initialize",
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
    fn wasm_spawn_invalid() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_spawn(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }
}
