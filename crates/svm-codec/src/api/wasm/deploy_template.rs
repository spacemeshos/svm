use super::wasm_buf_apply;

use crate::api::{self, json::JsonError};

///
/// Encodes a `deploy-template` json input into SVM `deploy-template` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_deploy_template(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, api::json::deploy_template)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Cursor;

    use svm_types::AppTemplate;

    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };
    use crate::template;

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

        let mut cursor = Cursor::new(&data[1..]);
        let actual = template::decode_deploy_template(&mut cursor).unwrap();

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
