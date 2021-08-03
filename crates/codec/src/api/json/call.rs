use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use std::io::Cursor;

use svm_types::Transaction;

use super::calldata::{decode_raw_input, DecodedCallData};
use super::serde_types::*;
use crate::api::json::{JsonError, JsonSerdeUtils};

/// Transforms a user-friendly `call` into an encoded form:
///
/// ```json
/// {
///   "version": 0,           // number
///   "target": "A2FB...",    // string
///   "func_name": "do_work", // string
///   "verifydata": "",       // string
///   "calldata": {
///     ...
///   }
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
    let encoded_bytes = encode_call_raw(json)?;
    Ok(EncodedData {
        data: HexBlob(encoded_bytes),
    }
    .to_json())
}

/// Much like [`encode_call`], but instead of returning a JSON wrapper it
/// returns the raw bytes.
pub fn encode_call_raw(json: &str) -> Result<Vec<u8>, JsonError> {
    let decoded_call = DecodedCall::from_json_str(json)?;
    let tx = Transaction::from(decoded_call);

    let mut buf = Vec::new();
    crate::call::encode_call(&tx, &mut buf);

    Ok(buf)
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
    let encoded_call = EncodedData::from_json_str(json)?;
    let tx = {
        let mut cursor = Cursor::new(&encoded_call.data.0[..]);

        crate::call::decode_call(&mut cursor).unwrap()
    };

    Ok(DecodedCall::from(tx).to_json())
}

#[derive(Clone, Serialize, Deserialize)]
struct DecodedCall {
    version: u16,
    target: AddressWrapper,
    func_name: String,
    verifydata: EncodedOrDecodedCalldata,
    calldata: EncodedOrDecodedCalldata,
}

impl JsonSerdeUtils for DecodedCall {}

impl From<DecodedCall> for Transaction {
    fn from(decoded: DecodedCall) -> Self {
        let target = decoded.target.into();

        Transaction {
            version: decoded.version,
            func_name: decoded.func_name,
            target,
            verifydata: decoded.verifydata.encode(),
            calldata: decoded.calldata.encode(),
        }
    }
}

impl From<Transaction> for DecodedCall {
    fn from(tx: Transaction) -> Self {
        DecodedCall {
            version: tx.version,
            target: AddressWrapper::from(&tx.target),
            func_name: tx.func_name.clone(),
            verifydata: EncodedOrDecodedCalldata::Decoded(
                DecodedCallData::new(&decode_raw_input(tx.verifydata()).unwrap().to_string())
                    .unwrap(),
            ),
            calldata: EncodedOrDecodedCalldata::Decoded(
                DecodedCallData::new(&decode_raw_input(tx.calldata()).unwrap().to_string())
                    .unwrap(),
            ),
        }
    }
}

/// This serves to provide an alternative to users between and decoded
/// `calldata`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum EncodedOrDecodedCalldata {
    Encoded(HexBlob<Vec<u8>>),
    Decoded(DecodedCallData),
}

impl EncodedOrDecodedCalldata {
    pub fn encode(self) -> Vec<u8> {
        match self {
            // It's encoded already.
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
        let verifydata = json::encode_calldata(
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
            "verifydata": verifydata["calldata"]
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
    fn json_call_valid() {
        let calldata = json::encode_calldata(
            &json!({
                "abi": ["i32", "i64"],
                "data": [10, 20],
            })
            .to_string(),
        )
        .unwrap();

        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            "verifydata": verifydata["verifydata"],
            "calldata": calldata["calldata"],
        })
        .to_string();

        let encoded_json = encode_call(&json).unwrap();
        let json = decode_call(&encoded_json.to_string()).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 0,
                "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
                "func_name": "do_something",
                "verifydata": {
                    "abi": ["bool", "i8"],
                    "data": [true, 3]
                },
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20]
                }
            })
        );
    }
}
