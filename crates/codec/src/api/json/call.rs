use serde::{Deserialize, Serialize};
use serde_json::{json, Value as Json};

use svm_abi_decoder::CallData;
use svm_types::Transaction;

use super::inputdata::DecodedInputData;
use super::serde_types::*;
use super::{get_field, parse_json, JsonError};
use crate::api::json::inputdata::calldata_to_json;
use crate::Codec;

/// Transforms a user-friendly `call` into an encoded form:
///
/// ```json
/// {
///   "version": 0,           // number
///   "target": "A2FB...",    // string
///   "func_name": "do_work", // string
///   "verifydata": {"abi": [], "data": []},
///   "calldata": {"abi": [], "data": []},
/// }
///
/// The `calldata` field can be both encoded and user-friendly form.
///
/// Result:
///
/// ```json
/// {
///   "data": "AABBCCFF81..."
/// }
/// ```
pub fn encode_call(json: &str) -> Result<Json, JsonError> {
    let json = &mut parse_json(json)?;

    let tx = Transaction {
        version: get_field(json, "version")?,
        target: get_field::<AddressWrapper>(json, "target")?.into(),
        func_name: get_field(json, "func_name")?,
        verifydata: get_field::<EncodedOrDecodedCalldata>(json, "verifydata")?.encode(),
        calldata: get_field::<EncodedOrDecodedCalldata>(json, "calldata")?.encode(),
    };

    Ok(json!({
        "data": HexBlob(tx.encode_to_vec())
    }))
}

/// Much like [`encode_call`], but instead of returning a JSON wrapper it
/// returns the raw bytes.
pub fn encode_call_raw(json: &str) -> Result<Vec<u8>, JsonError> {
    encode_call(json).map(|json| json.to_string().into_bytes())
}

/// Given a binary [`Transaction`] wrapped inside JSON,
/// Decodes it and returns a user-friendly JSON.
///
/// ```json
/// {
///   "data": "E9E50C787F2076BD5E44"
/// }
/// ```
pub fn decode_call(json: &str) -> Result<Json, JsonError> {
    let json = &mut parse_json(json)?;
    let data = get_field::<HexBlob<Vec<u8>>>(json, "data")?;
    let tx = Transaction::decode_bytes(data.0).map_err(|_| JsonError::InvalidField {
        path: "data".to_string(),
    })?;

    let verifydata = calldata_to_json(CallData::new(&tx.verifydata));
    let calldata = calldata_to_json(CallData::new(&tx.calldata));

    Ok(json!({
        "version": tx.version,
        "target": AddressWrapper::from(tx.target),
        "func_name": tx.func_name,
        "verifydata": verifydata,
        "calldata": calldata,
    }))
}

/// This serves to provide an alternative to users between and decoded
/// `calldata`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum EncodedOrDecodedCalldata {
    Encoded(HexBlob<Vec<u8>>),
    Decoded(DecodedInputData),
}

impl EncodedOrDecodedCalldata {
    pub fn encode(self) -> Vec<u8> {
        match self {
            Self::Encoded(encoded) => encoded.0,
            Self::Decoded(decoded) => decoded.encode().unwrap(),
        }
    }
}
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::json;

    #[test]
    fn json_call_missing_version() {
        let json = json!({}).to_string();

        let err = encode_call(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "version".to_string(),
            }
        );
    }

    #[test]
    fn json_call_missing_target() {
        let json = json!({
            "version": 0
        })
        .to_string();

        let err = encode_call(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "target".to_string(),
            }
        );
    }

    #[test]
    fn json_call_missing_func_name() {
        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        })
        .to_string();

        let err = encode_call(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "func_name".to_string(),
            }
        );
    }

    #[ignore]
    #[test]
    fn json_call_missing_verifydata() {
        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
        })
        .to_string();

        let err = encode_call(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "verifydata".to_string(),
            }
        );
    }

    #[test]
    fn json_call_missing_calldata() {
        let verifydata = json::encode_inputdata(
            &json!({
                "abi": ["bool", "i8"],
                "data": [true, 3],
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            "verifydata": verifydata["data"]
        })
        .to_string();

        let err = encode_call(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "calldata".to_string(),
            }
        );
    }

    #[test]
    fn encode_then_decode() {
        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            "calldata": {
                "abi": ["i32", "i64"],
                "data": [10, 20],
            },
            "verifydata": {
                "abi": ["bool", "i8"],
                "data": [true, 3],
            }
        })
        .to_string();

        let encoded = encode_call(json.as_str()).unwrap();
        let decoded = decode_call(encoded.to_string().as_str()).unwrap();

        assert_eq!(json, decoded.to_string());
    }
}
