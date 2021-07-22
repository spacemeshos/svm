use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use std::io::Cursor;

use svm_types::{AccountAddr, Transaction};

use super::wrappers::*;
use crate::api::json::{JsonError, JsonSerdeUtils};
use crate::call;

///
/// ```json
/// {
///   "version": 0,           // number
///   "target": "A2FB...",    // string
///   "func_name": "do_work", // string
///   "verifydata": "",       // string
///   "calldata": "",         // string
/// }
/// ```
pub fn json_call_to_bytes(json: &str) -> Result<Vec<u8>, JsonError> {
    let decoded_call = DecodedCall::from_json_str(json)?;
    let tx = Transaction::from(decoded_call);

    let mut buf = Vec::new();
    call::encode_call(&tx, &mut buf);
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
pub fn unwrap_binary_json_call(json: &str) -> Result<Json, JsonError> {
    let encoded_call = EncodedCall::from_json_str(json)?;
    let tx = {
        let mut cursor = Cursor::new(&encoded_call.data.0[..]);
        call::decode_call(&mut cursor).unwrap()
    };

    // let verifydata = json::bytes_to_str(&tx.verifydata);
    // let verifydata = json::decode_calldata(&json!({ "calldata": verifydata }))?;

    Ok(DecodedCall::from(tx).to_json())
}

#[derive(Clone, Serialize, Deserialize)]
struct DecodedCall {
    version: u16,
    target: AddressWrapper,
    func_name: String,
    // verifydata: String,
    calldata: HexBlob<Vec<u8>>,
}

impl DecodedCall {
    fn account_addr(&self) -> AccountAddr {
        AccountAddr::new(self.target.0.clone())
    }
}

impl JsonSerdeUtils for DecodedCall {}

impl From<DecodedCall> for Transaction {
    fn from(decoded: DecodedCall) -> Self {
        let target = decoded.account_addr();
        Transaction {
            version: decoded.version,
            func_name: decoded.func_name,
            target,
            calldata: decoded.calldata.0,
        }
    }
}

impl From<Transaction> for DecodedCall {
    fn from(tx: Transaction) -> Self {
        DecodedCall {
            version: tx.version,
            target: AddressWrapper(tx.target.inner().clone()),
            func_name: tx.func_name.clone(),
            calldata: HexBlob(tx.calldata),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EncodedCall {
    data: HexBlob<Vec<u8>>,
}

impl JsonSerdeUtils for EncodedCall {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::json;

    #[test]
    fn json_call_missing_version() {
        let json = json!({}).to_string();

        let err = json_call_to_bytes(&json).unwrap_err();
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

        let err = json_call_to_bytes(&json).unwrap_err();
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

        let err = json_call_to_bytes(&json).unwrap_err();
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

        let err = json_call_to_bytes(&json).unwrap_err();
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

        let err = json_call_to_bytes(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "calldata".to_string(),
            }
        );
    }

    #[test]
    fn json_call_valid() {
        let _verifydata = json::encode_calldata(
            &json!({
                "abi": ["bool", "i8"],
                "data": [true, 3],
            })
            .to_string(),
        )
        .unwrap();

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
            // "verifydata": verifydata["calldata"],
            "calldata": calldata["calldata"],
        })
        .to_string();

        let bytes = json_call_to_bytes(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = unwrap_binary_json_call(&json!({ "data": data }).to_string()).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 0,
                "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
                "func_name": "do_something",
                // "verifydata": {
                //     "abi": ["bool", "i8"],
                //     "data": [true, 3]
                // },
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20]
                }
            })
        );
    }
}
