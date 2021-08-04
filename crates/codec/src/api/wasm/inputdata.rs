use super::wasm_buf_apply;
use crate::{api, api::json::JsonError};

/// Given an offset to a Wasm buffer holding the data to be encoded,
/// encodes it and returns an offset to the encoded binary `Input Data` (wrapped within a JSON).
pub fn encode_inputdata(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &str| {
        let json = api::json::encode_inputdata(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

/// Given an offset to a Wasm buffer holding a binary `Input Data`,
/// decodes it and returns an offset to be decoded `Input Data` (wrapped within a JSON)
pub fn decode_inputdata(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &str| {
        let json = api::json::decode_inputdata(json)?;

        Ok(api::json::to_bytes(&json))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use serde_json::{json, Value as Json};

    fn wasm_buf_as_json(buf_ptr: usize) -> Json {
        let data = wasm_buffer_data(buf_ptr);
        assert_eq!(data[0], BUF_OK_MARKER);

        let s = unsafe { String::from_utf8_unchecked(data[1..].to_vec()) };
        let json: Json = serde_json::from_str(&s).unwrap();

        json
    }

    #[test]
    fn wasm_encode_inputdata_valid() {
        let json = r#"{
          "abi": ["i32", "address"],
          "data": [10, "102030405060708090A011121314151617181920"]
        }"#;

        // encode
        let json_buf = to_wasm_buffer(json.as_bytes());
        let inputdata = encode_inputdata(json_buf).unwrap();
        let data = wasm_buffer_data(inputdata);
        assert_eq!(data[0], BUF_OK_MARKER);

        // decode
        let data_buf = to_wasm_buffer(&data[1..]);
        let res_buf = decode_inputdata(data_buf).unwrap();

        assert_eq!(
            wasm_buf_as_json(res_buf),
            json!({
              "abi": ["i32", "address"],
              "data": [10, "102030405060708090A011121314151617181920"]
            })
        );

        free(json_buf);
        free(inputdata);
        free(data_buf);
        free(res_buf);
    }

    #[test]
    fn wasm_encode_inputdata_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_inputdata(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_decode_inputdata_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = decode_inputdata(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert_eq!(error, "The given JSON is syntactically invalid due to EOF.");

        free(json_buf);
        free(error_buf);
    }
}
