use serde_json::{json, Value};

use crate::{
    api::json::{self, JsonError},
    api::raw,
    app,
    nibble::{NibbleIter, NibbleWriter},
};

use svm_types::{App, SpawnApp};

///
/// ```json
/// {
///   version: 0,           // number
///   template: 'A2FB...',  // string
///   ctor_index: 0,        // number
///   ctor_buf: '',         // string
///   ctor_args: ['10i32', '20i64', ...] // Array of `String`
/// }
/// ```
pub fn spawn_app(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")?;
    let template = json::as_addr(json, "template")?.into();
    let ctor_idx = json::as_u16(json, "ctor_index")?;

    let ctor_buf = json::as_string(json, "ctor_buf")?;
    let ctor_buf = json::str_to_bytes(&ctor_buf, "ctor_buf")?;

    let ctor_args = json::as_string(json, "ctor_args")?;
    let ctor_args = json::str_to_bytes(&ctor_args, "ctor_args")?;

    let mut iter = NibbleIter::new(&ctor_args);
    let ctor_args = raw::decode_func_args(&mut iter).unwrap();

    let spawn = SpawnApp {
        app: App { version, template },
        ctor_idx,
        ctor_args,
        ctor_buf,
    };

    let mut w = NibbleWriter::new();
    app::encode_spawn_app(&spawn, &mut w);

    let bytes = w.into_bytes();
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use svm_types::{Address, WasmValue};

    use crate::nibble::NibbleIter;

    #[test]
    fn json_spawn_app_missing_version() {
        let json = json!({});

        let err = spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "version".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_template_addr() {
        let json = json!({
            "version": 0
        });

        let err = spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "template".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_index() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let err = spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "ctor_index".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_buf() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "ctor_index": 0,
        });

        let err = spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "ctor_buf".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_args() {
        let calldata = json::encode_calldata(&json!({
            "abi": [],
            "data": []
        }))
        .unwrap();

        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "ctor_index": 0,
            "ctor_buf": calldata["func_buf"]
        });

        let err = spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "ctor_args".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_valid() {
        let calldata = json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20]
        }))
        .unwrap();

        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "ctor_index": 1,
            "ctor_buf": calldata["func_buf"],
            "ctor_args": calldata["func_args"]
        });

        let bytes = spawn_app(&json).unwrap();

        let mut iter = NibbleIter::new(&bytes[..]);
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
            ctor_buf: vec![],
            ctor_args: vec![WasmValue::I32(10), WasmValue::I64(20)],
        };

        assert_eq!(actual, expected);
    }
}
