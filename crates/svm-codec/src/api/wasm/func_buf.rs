use serde_json::{self as json, Value};

use svm_types::{Address, AppTemplate, WasmValue};

use super::{
    alloc, error::into_error_buffer, free, to_wasm_buffer, wasm_buf_apply, wasm_buf_data_copy,
    wasm_buffer_data, BUF_ERROR_MARKER, BUF_OK_MARKER,
};
use crate::{api, api::json::JsonError, app, nibble::NibbleWriter};

pub fn encode_func_buf(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, api::json::encode_func_buf)
}

pub fn decode_func_buf(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, |json: &Value| {
        let s: String = api::json::decode_func_buf(json)?;

        Ok(s.into_bytes())
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::wasm::error_as_string;
    use crate::nibble::NibbleIter;

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
    fn wasm_encode_func_buf_valid() {
        let json = r#"{
          "abi": ["address"],
          "data": ["102030405060708090A011121314151617181920"]
        }"#;

        // encode
        let json_buf = to_wasm_buffer(json.as_bytes());
        let func_buf = encode_func_buf(json_buf).unwrap();
        let data = wasm_buffer_data(func_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        // decode
        let json = json!({
            "data": fmt_hex(&data[1..], "")
        })
        .to_string();

        let data_buf = to_wasm_buffer(json.as_bytes());
        let res_buf = decode_func_buf(data_buf).unwrap();

        assert_eq!(
            wasm_buf_as_json(res_buf),
            json!([{
                "address": "102030405060708090a011121314151617181920"
            }])
        );

        free(json_buf);
        free(func_buf);
        free(data_buf);
        free(res_buf);
    }

    #[test]
    fn wasm_encode_func_buf_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_func_buf(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }

    #[test]
    fn wasm_decode_func_buf_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = decode_func_buf(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }
}
