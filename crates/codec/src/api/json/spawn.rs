use std::io::Cursor;

use svm_types::{Account, SpawnAccount};

use serde_json::{json, Value};

use crate::api::json::{self, JsonError};

use crate::spawn;

///
/// ```json
/// {
///   version: 0,              // number
///   template: 'A2FB...',     // string
///   name: 'My Account',      // string
///   ctor_name: 'initialize', // number
///   calldata: '',            // string
/// }
/// ```
pub fn encode_spawn(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")? as u16;
    let template = json::as_addr(json, "template")?.into();
    let name = json::as_string(json, "name")?;
    let ctor_name = json::as_string(json, "ctor_name")?;

    let calldata = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&calldata, "calldata")?;

    let spawn = SpawnAccount {
        version,
        account: Account::new(template, name),
        ctor_name,
        calldata,
    };

    let mut buf = Vec::new();
    spawn::encode(&spawn, &mut buf);

    Ok(buf)
}

/// Given a binary [`SpawnAccount`] transaction wrapped inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_spawn(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    let mut cursor = Cursor::new(&bytes[..]);
    let spawn = spawn::decode(&mut cursor).unwrap();

    let version = spawn.version;
    let ctor_name = spawn.ctor_name;
    let template = json::addr_to_str(&spawn.account.template_addr.inner());

    let calldata = json::bytes_to_str(&spawn.calldata);
    let calldata = json::decode_calldata(&json!({ "calldata": calldata }))?;

    let name = spawn.account.name;

    let json = json!({
        "version": version,
        "template": template,
        "name": name,
        "ctor_name": ctor_name,
        "calldata": calldata,
    });

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_spawn_app_missing_version() {
        let json = json!({});

        let err = encode_spawn(&json).unwrap_err();
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

        let err = encode_spawn(&json).unwrap_err();
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

        let err = encode_spawn(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "name".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_name() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My App",
        });

        let err = encode_spawn(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "ctor_name".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_app_missing_ctor_buf() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My Account",
            "ctor_name": "initialize",
        });

        let err = encode_spawn(&json).unwrap_err();
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
        let calldata = json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20]
        }))
        .unwrap();

        let json = json!({
            "version": 1,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My Account",
            "ctor_name": "initialize",
            "calldata": calldata["calldata"],
        });

        let bytes = encode_spawn(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = decode_spawn(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 1,
                "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
                "name": "My Account",
                "ctor_name": "initialize",
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20]
                }
            })
        );
    }
}
