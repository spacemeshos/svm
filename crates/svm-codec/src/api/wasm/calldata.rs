use serde_json::Value;

use super::wasm_buf_apply;
use crate::{api, api::json::JsonError};

/// Given an offset to a Wasm buffer holding the data to be encoded
/// to a binary `Calldata`, encodes it and returns an offset to the encoded
/// binary `Calldata` (wrapped within a JSON).
pub fn encode_calldata(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &Value| {
        let json = api::json::encode_calldata(json)?;

        api::json::to_bytes(&json)
    })
}

/// Given an offset to a Wasm buffer holding a binary `Calldata`,
/// decodes it and returns an offset to be decoded `Calldata` (wrapped within a JSON)
pub fn decode_calldata(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &Value| {
        let json = api::json::decode_calldata(json)?;

        api::json::to_bytes(&json)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use serde_json::json;
    use svm_common::fmt::fmt_hex;

    fn wasm_buf_as_json(buf_ptr: usize) -> Value {
        let data = wasm_buffer_data(buf_ptr);
        assert_eq!(data[0], BUF_OK_MARKER);

        let s = unsafe { String::from_utf8_unchecked(data[1..].to_vec()) };
        let json: Value = serde_json::from_str(&s).unwrap();

        json
    }

    #[test]
    fn wasm_encode_calldata_valid() {
        let json = r#"{
          "abi": ["i32", "address"],
          "data": [10, "102030405060708090A011121314151617181920"]
        }"#;

        // encode
        let json_buf = to_wasm_buffer(json.as_bytes());
        let calldata = encode_calldata(json_buf).unwrap();
        let data = wasm_buffer_data(calldata);
        assert_eq!(data[0], BUF_OK_MARKER);

        // decode
        let data_buf = to_wasm_buffer(&data[1..]);
        let res_buf = decode_calldata(data_buf).unwrap();

        assert_eq!(
            wasm_buf_as_json(res_buf),
            json!({
              "abi": ["i32", "address"],
              "data": [10, "102030405060708090A011121314151617181920"]
            })
        );

        free(json_buf);
        free(calldata);
        free(data_buf);
        free(res_buf);
    }

    #[test]
    fn wasm_encode_calldata_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_calldata(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_decode_calldata_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = decode_calldata(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }
}
