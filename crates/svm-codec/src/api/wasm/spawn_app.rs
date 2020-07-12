use super::wasm_buf_apply;
use crate::{api, api::json::JsonError};

/// Encodes a `spawn-app` json input into SVM `spawn-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
pub fn encode_spawn_app(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, api::json::spawn_app)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nibble::NibbleIter;

    use crate::api::wasm::error_as_string;

    use serde_json::json;

    #[test]
    fn wasm_encode_spawn_app_valid() {
        let json = r#"{
          "version": 0,
          "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
          "ctor_index": 1,
          "ctor_buf": "A2B3",
          "ctor_args": ["10i32", "20i64"]
        }"#;

        let json_buf = to_wasm_buffer(json.as_bytes());
        let tx_buf = encode_spawn_app(json_buf).unwrap();

        let data = wasm_buffer_data(tx_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let mut iter = NibbleIter::new(&data[1..]);
        let actual = crate::api::raw::decode_spawn_app(&mut iter).unwrap();

        let addr_bytes = vec![
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0x00, 0xA0, 0xB0, 0xC0, 0xD0,
            0xE0, 0xF0, 0xAB, 0xCD, 0xEF, 0xFF,
        ];

        let expected = SpawnApp {
            app: App {
                version: 0,
                template: Address::from(&addr_bytes[..]).into(),
            },
            ctor_idx: 1,
            ctor_buf: vec![0xA2, 0xB3],
            ctor_args: vec![WasmValue::I32(10), WasmValue::I64(20)],
        };

        assert_eq!(actual, expected);

        free(json_buf);
        free(tx_buf);
    }

    #[test]
    fn wasm_encode_spawn_app_invalid_json() {
        let json = "{";

        let json_buf = to_wasm_buffer(json.as_bytes());
        let error_buf = encode_spawn_app(json_buf).unwrap();

        let error = unsafe { error_as_string(error_buf) };

        assert!(error.starts_with(r#"Error("EOF while parsing"#));

        free(json_buf);
        free(error_buf);
    }
}
