use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::io::Cursor;

use svm_types::{Account, SpawnAccount, TemplateAddr};

use super::wrappers::AddressWrapper;
use super::TypeInformation;
use crate::api::json::{self, HexBlob, JsonError};
use crate::spawn;

///
/// ```json
/// {
///   "version": 0,              // number
///   "template": "A2FB...",     // string
///   "name": "My Account",      // string
///   "ctor_name": "initialize", // number
///   "calldata": "",            // string
/// }
/// ```
pub fn encode_spawn(json: Value) -> Result<Vec<u8>, JsonError> {
    let wrapper = SpawnWrapper::new(json)?;

    let mut buf = Vec::new();
    spawn::encode(&SpawnAccount::from(wrapper), &mut buf);
    Ok(buf)
}

/// Given a binary [`SpawnAccount`] transaction wrapped inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_spawn(json: Value) -> Result<Value, JsonError> {
    let data = json::as_string(&json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    let mut cursor = Cursor::new(&bytes[..]);
    let spawn = spawn::decode(&mut cursor).unwrap();

    Ok(serde_json::to_value(SpawnWrapper::from(spawn)).unwrap())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SpawnWrapper {
    version: u16,
    #[serde(rename = "template")]
    template_addr: AddressWrapper,
    name: String,
    ctor_name: String,
    calldata: HexBlob,
}

impl SpawnWrapper {
    fn new(json: Value) -> Result<Self, JsonError> {
        serde_json::from_value(json).map_err(JsonError::from_serde::<Self>)
    }
}

impl From<SpawnAccount> for SpawnWrapper {
    fn from(spawn: SpawnAccount) -> Self {
        let template_addr = AddressWrapper(spawn.account.template_addr().inner().clone());

        Self {
            version: spawn.version,
            name: spawn.account.name,
            template_addr,
            ctor_name: spawn.ctor_name,
            calldata: HexBlob(spawn.calldata),
        }
    }
}

impl From<SpawnWrapper> for SpawnAccount {
    fn from(wrapper: SpawnWrapper) -> Self {
        let template_addr = TemplateAddr::new(wrapper.template_addr.0);

        SpawnAccount {
            version: wrapper.version,
            account: Account::new(template_addr, wrapper.name),
            ctor_name: wrapper.ctor_name,
            calldata: wrapper.calldata.0,
        }
    }
}

impl TypeInformation for SpawnWrapper {
    fn type_of_field_as_str(field: &str) -> Option<&str> {
        Some(match field {
            "version" => "number",
            _ => "string",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_spawn_missing_version() {
        let json = json!({});
        let err = encode_spawn(json).unwrap_err();

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
        let err = encode_spawn(json).unwrap_err();

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
        let err = encode_spawn(json).unwrap_err();

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
        let err = encode_spawn(json).unwrap_err();

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
        let err = encode_spawn(json).unwrap_err();

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
        let calldata = json::encode_calldata(json!({
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

        let bytes = encode_spawn(json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = decode_spawn(json!({ "data": data })).unwrap();

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
