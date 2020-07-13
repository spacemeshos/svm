use super::wasm_buf_apply;
use crate::{api, api::json::JsonError};

///
/// Encodes a `exec-app` json input into SVM `exec-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
///
/// See also: `alloc` and `free`
///
pub fn encode_exec_app(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, api::json::exec_app)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nibble::NibbleIter;

    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use svm_types::{Address, AppTransaction, WasmValue};

    #[test]
    fn wasm_encode_exec_app_valid() {
        let json = r#"{
          "version": 0,
          "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
          "func_index": 1,
          "func_buf": "A2B3",
          "func_args": ["10i32", "20i64"]
        }"#;

        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_exec_app(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let mut iter = NibbleIter::new(&data[1..]);
        let actual = crate::api::raw::decode_exec_app(&mut iter).unwrap();

        let addr_bytes = vec![
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0x00, 0xA0, 0xB0, 0xC0, 0xD0,
            0xE0, 0xF0, 0xAB, 0xCD, 0xEF, 0xFF,
        ];

        let expected = AppTransaction {
            version: 0,
            app: Address::from(&addr_bytes[..]).into(),
            func_idx: 1,
            func_buf: vec![0xA2, 0xB3],
            func_args: vec![WasmValue::I32(10), WasmValue::I64(20)],
        };

        assert_eq!(actual, expected);

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
