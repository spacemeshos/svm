use serde_json::{json, Value as Json};

use svm_types::{Account, SpawnAccount};

use super::call::EncodedOrDecodedCalldata;
use super::inputdata::decode_raw_input;
use super::serde_types::{HexBlob, TemplateAddrWrapper};
use super::{get_field, parse_json, JsonError};
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
    let json = &mut parse_json(json)?;

    let version = get_field(json, "version")?;
    let template_addr = get_field::<TemplateAddrWrapper>(json, "template")?.into();
    let name = get_field(json, "name")?;
    let ctor_name = get_field(json, "ctor_name")?;
    let calldata = get_field::<EncodedOrDecodedCalldata>(json, "calldata")?.encode();

    Ok(SpawnAccount {
        version,
        ctor_name,
        account: Account::new(template_addr, name),
        calldata,
    }
    .encode_to_vec())
}

/// Given a binary [`SpawnAccount`] transaction wrapped inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_spawn(json: &str) -> Result<Json, JsonError> {
    let json = &mut parse_json(json)?;
    let encoded_data = get_field::<HexBlob<Vec<u8>>>(json, "data")?.0;

    let spawn = SpawnAccount::decode_bytes(encoded_data).unwrap();
    let decoded_calldata = decode_raw_input(&spawn.calldata).unwrap();

    Ok(json!({
        "version": spawn.version,
        "template": TemplateAddrWrapper::from(spawn.template_addr().clone()),
        "name": spawn.account.name,
        "ctor_name": spawn.ctor_name,
        "calldata": decoded_calldata,
    }))
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
