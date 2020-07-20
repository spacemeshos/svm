use serde_json::{json, Value};

use crate::{
    api::json::{self, JsonError},
    api::raw,
    app,
    nibble::{NibbleIter, NibbleWriter},
};

use svm_types::{AddressOf, App, SpawnApp, WasmValue};

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
pub fn encode_spawn_app(json: &Value) -> Result<Vec<u8>, JsonError> {
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

pub fn decode_spawn_app(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    let mut iter = NibbleIter::new(&bytes);
    let spawn = raw::decode_spawn_app(&mut iter).unwrap();

    let version = spawn.app.version;
    let ctor_idx = spawn.ctor_idx;
    let template = json::addr_to_str(&spawn.app.template.inner());

    let ctor_buf = json::bytes_to_str(&spawn.ctor_buf);
    let ctor_buf = json::decode_func_buf(&json!({ "data": ctor_buf }))?;
    let ctor_args = json::wasm_values_to_json(&spawn.ctor_args);

    let json = json!({
        "version": version,
        "template": template,
        "ctor_index": ctor_idx,
        "ctor_buf": ctor_buf,
        "ctor_args": ctor_args
    });

    Ok(json)
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

        let err = encode_spawn_app(&json).unwrap_err();
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

        let err = encode_spawn_app(&json).unwrap_err();
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

        let err = encode_spawn_app(&json).unwrap_err();
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

        let err = encode_spawn_app(&json).unwrap_err();
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

        let err = encode_spawn_app(&json).unwrap_err();
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
        let template_addr = "1122334455667788990011223344556677889900";

        let calldata = json::encode_calldata(&json!({
            "abi": ["i32", "address", "i64"],
            "data": [10, template_addr, 20]
        }))
        .unwrap();

        let json = json!({
            "version": 1,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "ctor_index": 2,
            "ctor_buf": calldata["func_buf"],
            "ctor_args": calldata["func_args"]
        });

        let bytes = encode_spawn_app(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = decode_spawn_app(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
                "ctor_index": 2,
                "ctor_buf": [{"address": template_addr}],
                "ctor_args": ["10i32", "20i64"],
            })
        );
    }
}
