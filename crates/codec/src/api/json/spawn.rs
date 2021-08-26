use serde::{Deserialize, Serialize};
use serde_json::Value;

use svm_types::{Account, SpawnAccount};

use super::call::EncodedOrDecodedCalldata;
use super::inputdata::DecodedInputData;
use super::serde_types::{EncodedData, TemplateAddrWrapper};
use super::{JsonError, JsonSerdeUtils};
use crate::Codec;

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
    let spawn = SpawnAccount::from(decoded);

    Ok(spawn.encode_to_vec())
}

/// Given a binary [`SpawnAccount`] transaction wrapped inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_spawn(json: &str) -> Result<Value, JsonError> {
    let encoded_spawn = EncodedData::from_json_str(json)?;

    let spawn = SpawnAccount::decode_bytes(encoded_spawn.data.0).unwrap();

    Ok(DecodedSpawn::from(spawn).to_json())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DecodedSpawn {
    version: u16,
    #[serde(rename = "template")]
    template_addr: TemplateAddrWrapper,
    name: String,
    ctor_name: String,
    calldata: EncodedOrDecodedCalldata,
}

impl JsonSerdeUtils for DecodedSpawn {}

impl From<SpawnAccount> for DecodedSpawn {
    fn from(spawn: SpawnAccount) -> Self {
        let template_addr = TemplateAddrWrapper(spawn.template_addr().clone());
        let decoded_calldata = super::inputdata::decode_raw_input(&spawn.calldata).unwrap();

        Self {
            version: spawn.version,
            name: spawn.account.name,
            template_addr,
            ctor_name: spawn.ctor_name,
            calldata: EncodedOrDecodedCalldata::Decoded(
                DecodedInputData::new(&decoded_calldata.to_string())
                    .expect("Invalid JSON immediately after serialization"),
            ),
        }
    }
}

impl From<DecodedSpawn> for SpawnAccount {
    fn from(wrapper: DecodedSpawn) -> Self {
        let template_addr = wrapper.template_addr.0;

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
    use crate::api::json::serde_types::HexBlob;

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
            "name": "My Account",
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
        let calldata = json::encode_inputdata(
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
