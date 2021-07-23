use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::io::Cursor;

use svm_types::{Account, SpawnAccount, TemplateAddr};

use super::call::EncodedOrDecodedCalldata;
use super::calldata::DecodedCallData;
use super::serde_types::{AddressWrapper, EncodedData, HexBlob};
use super::JsonSerdeUtils;
use crate::api::json::JsonError;
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
pub fn encode_spawn(json: &str) -> Result<Vec<u8>, JsonError> {
    let decoded = DecodedSpawn::from_json_str(json)?;
    let spawn = decoded.into();

    let mut buf = Vec::new();
    spawn::encode(&spawn, &mut buf);
    Ok(buf)
}

/// Given a binary [`SpawnAccount`] transaction wrapped inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_spawn(json: &str) -> Result<Value, JsonError> {
    let encoded_spawn = EncodedData::from_json_str(json)?;

    let mut cursor = Cursor::new(&encoded_spawn.data.0[..]);
    let spawn = spawn::decode(&mut cursor).unwrap();

    Ok(DecodedSpawn::from(spawn).to_json())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DecodedSpawn {
    version: u16,
    #[serde(rename = "template")]
    template_addr: AddressWrapper,
    name: String,
    ctor_name: String,
    calldata: EncodedOrDecodedCalldata,
}

impl JsonSerdeUtils for DecodedSpawn {}

impl From<SpawnAccount> for DecodedSpawn {
    fn from(spawn: SpawnAccount) -> Self {
        let template_addr = AddressWrapper(spawn.account.template_addr().inner().clone());
        let encoded_calldata = super::calldata::encode_calldata(
            &EncodedData {
                data: HexBlob(spawn.calldata),
            }
            .to_json()
            .to_string(),
        )
        .unwrap()
        .to_string();
        let calldata = DecodedCallData::from_json_str(&encoded_calldata)
            .expect("Invalid JSON immediately after serialization");

        Self {
            version: spawn.version,
            name: spawn.account.name,
            template_addr,
            ctor_name: spawn.ctor_name,
            calldata: EncodedOrDecodedCalldata::Decoded(calldata),
        }
    }
}

impl From<DecodedSpawn> for SpawnAccount {
    fn from(wrapper: DecodedSpawn) -> Self {
        let template_addr = TemplateAddr::new(wrapper.template_addr.0);

        SpawnAccount {
            version: wrapper.version,
            account: Account::new(template_addr, wrapper.name),
            ctor_name: wrapper.ctor_name,
            calldata: wrapper.calldata.encode(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::json;

    #[test]
    fn json_spawn_missing_version() {
        let json = json!({}).to_string();
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "version".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_template_addr() {
        let json = json!({
            "version": 0
        })
        .to_string();
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "template".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_name() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        })
        .to_string();
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "name".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_missing_ctor_name() {
        let json = json!({
            "version": 0,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My App",
        })
        .to_string();
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "ctor_name".to_string(),
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
        })
        .to_string();
        let err = encode_spawn(&json).unwrap_err();

        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "calldata".to_string(),
            }
        );
    }

    #[test]
    fn json_spawn_valid() {
        let calldata = json::encode_calldata(
            &json!({
                "abi": ["i32", "i64"],
                "data": [10, 20]
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
            "version": 1,
            "template": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "name": "My Account",
            "ctor_name": "initialize",
            "calldata": calldata["data"],
        })
        .to_string();
        println!("SPAWNING {}", json);

        let bytes = encode_spawn(&json).unwrap();
        let data = HexBlob(&bytes);
        let json = decode_spawn(&json!({ "data": data }).to_string()).unwrap();

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
