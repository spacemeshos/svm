use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::io::Cursor;

use svm_types::{Account, SpawnAccount};

use super::TypeInformation;
use crate::api::json::{self, JsonError};
use crate::spawn;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SpawnJsonlike {
    version: u16,
    template: String,
    name: String,
    ctor_name: String,
    calldata: String,
}

impl TypeInformation for SpawnJsonlike {
    fn type_of_field_as_str(field: &str) -> Option<&str> {
        Some(match field {
            "version" => "number",
            _ => "string",
        })
    }
}

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
    let jsonlike: SpawnJsonlike = serde_json::from_value(json.clone())
        .map_err(|e| JsonError::from_serde::<SpawnJsonlike>(e))?;

    let template = json::as_addr(json, "template")?.into();

    let spawn = SpawnAccount {
        version: jsonlike.version,
        account: Account::new(template, jsonlike.name),
        ctor_name: jsonlike.ctor_name,
        calldata: jsonlike.calldata.into_bytes(),
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
    fn json_spawn_missing_version() {
        let json = json!({});
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "version".to_string(),
                reason: "value `null` isn\'t a(n) number".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_template_addr() {
        let json = json!({
            "version": 0
        });
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "template".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_name() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "name".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_ctor_name() {
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
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_ctor_buf() {
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
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_valid() {
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
