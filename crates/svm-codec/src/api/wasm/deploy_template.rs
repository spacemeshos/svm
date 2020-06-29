use serde_json::{self as json, Value};

use svm_types::{Address, AppTemplate, WasmValue};

use super::{
    alloc, error::into_error_buffer, free, to_wasm_buffer, wasm_buf_data_copy, wasm_buffer_data,
    BUF_ERROR_MARKER, BUF_OK_MARKER,
};
use crate::{api, api::json::JsonError, app, nibble::NibbleWriter};

///
/// Encodes a `deploy-template` json input into SVM `deploy-template` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_deploy_template(ptr: usize) -> Result<usize, JsonError> {
    let bytes = wasm_buffer_data(ptr);
    let json: json::Result<Value> = serde_json::from_slice(bytes);

    match json {
        Ok(ref json) => {
            let bytes = api::json::deploy_template(&json)?;

            let mut buf = Vec::with_capacity(1 + bytes.len());
            buf.push(BUF_OK_MARKER);
            buf.extend_from_slice(&bytes);

            let ptr = to_wasm_buffer(&buf);
            Ok(ptr)
        }
        Err(err) => {
            let ptr = into_error_buffer(err);

            Ok(ptr)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nibble::NibbleIter;

    use crate::api::wasm::error_as_string;

    use serde_json::json;

    #[test]
    fn wasm_encode_deploy_template_valid() {
        let json = r#"{
          "version": 0,
          "name": "My Template",
          "code": "C0DE",
          "data": "0000000100000003"
        }"#;

        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_deploy_template(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let mut iter = NibbleIter::new(&data[1..]);
        let actual = crate::api::raw::decode_deploy_template(&mut iter).unwrap();

        let expected = AppTemplate {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0xC0, 0xDE],
            data: vec![1, 3].into(),
        };

        assert_eq!(actual, expected);

        free(json_buf);
        free(tx_buf);
    }

    #[test]
    fn wasm_encode_deploy_template_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_deploy_template(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }
}
