use serde_json::Value;

use crate::{
    api::json::{self, JsonError},
    nibble::NibbleWriter,
    transaction,
};

use svm_types::{Address, AppTransaction, WasmValue};

///
/// ```json
/// {
///   version: 0,      // number
///   app: 'A2FB...',  // string
///   func_index: 0,   // number
///   func_buf: '',   // string
///   func_args: ['10i32', '20i64', ...] // Array of `string`
/// }
/// ```
pub fn exec_app(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")?;
    let app = json::as_addr(json, "app")?.into();
    let func_idx = json::as_u16(json, "func_index")?;
    let func_buf = json::as_blob(json, "func_buf")?;
    let func_args = json::as_wasm_values(json, "func_args")?;

    let tx = AppTransaction {
        version,
        app,
        func_idx,
        func_args,
        func_buf,
    };

    let mut w = NibbleWriter::new();
    transaction::encode_exec_app(&tx, &mut w);

    let bytes = w.into_bytes();
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use crate::nibble::NibbleIter;

    #[ignore]
    fn json_exec_app_missing_version() {
        let json = json!({});

        let err = exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "version".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_app_addr() {
        let json = json!({
            "version": 0
        });

        let err = exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "app".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_func_index() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let err = exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "func_index".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_func_buf() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_index": 0,
        });

        let err = exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "func_buf".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_func_args() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_index": 0,
            "func_buf": "0000"
        });

        let err = exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "func_args".to_string(),
                reason: "value `null` isn\'t an array".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_valid() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_index": 1,
            "func_buf": "A2B3",
            "func_args": ["10i32", "20i64"]
        });

        let bytes = exec_app(&json).unwrap();

        let mut iter = NibbleIter::new(&bytes[..]);
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
    }
}
