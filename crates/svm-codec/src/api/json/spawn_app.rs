use svm_nibble::{NibbleIter, NibbleWriter};
use svm_types::{AddressOf, App, SpawnApp, WasmValue};

use serde_json::{json, Value};

use crate::{
    api::json::{self, JsonError},
    api::raw,
    app,
};

///
/// ```json
/// {
///   version: 0,           // number
///   template: 'A2FB...',  // string
///   name: 'My App',       // string
///   ctor: 'initialize',   // number
///   calldata: '',         // string
/// }
/// ```
pub fn encode_spawn_app(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")?;
    let template = json::as_addr(json, "template")?.into();
    let name = json::as_string(json, "name")?;
    let ctor = json::as_string(json, "ctor")?;

    let calldata = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&calldata, "calldata")?;

    let spawn = SpawnApp {
        app: App {
            version,
            name,
            template,
        },
        ctor,
        calldata,
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
    let ctor = spawn.ctor;
    let template = json::addr_to_str(&spawn.app.template.inner());

    let calldata = json::bytes_to_str(&spawn.calldata);
    let calldata = json::decode_calldata(&json!({ "calldata": calldata }))?;

    let name = spawn.app.name;

    let json = json!({
        "version": version,
        "template": template,
        "name": name,
        "ctor": ctor,
        "calldata": calldata,
    });

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use svm_nibble::NibbleIter;
    use svm_types::{Address, WasmValue};

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
    fn json_spawn_app_missing_name() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let err = encode_spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "name".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_index() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My App",
        });

        let err = encode_spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "ctor".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_buf() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My App",
            "ctor": 0,
        });

        let err = encode_spawn_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "calldata".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_valid() {
        let template_addr = "1122334455667788990011223344556677889900";

        let calldata = json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20]
        }))
        .unwrap();

        let json = json!({
            "version": 1,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My App",
            "ctor": 2,
            "calldata": calldata["calldata"],
        });

        let bytes = encode_spawn_app(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = decode_spawn_app(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
                "name": "My App",
                "ctor": 2,
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20]
                }
            })
        );
    }
}
