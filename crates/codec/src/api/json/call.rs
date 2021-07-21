use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use std::io::Cursor;

use svm_types::{AccountAddr, Transaction};

use super::wrappers::*;
use crate::api::json::{JsonError, TypeInformation};
use crate::call;

#[derive(Clone, Serialize, Deserialize)]
struct CallWrapper {
    version: u16,
    target: AddressWrapper,
    func_name: String,
    // verifydata: String,
    calldata: HexBlob<Vec<u8>>,
}

impl TypeInformation for CallWrapper {
    fn type_of_field_as_str(field: &str) -> Option<&str> {
        Some(match field {
            "version" => "number",
            "func_name" | "target" | "calldata" => "string",
            _ => unreachable!(),
        })
    }
}

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
pub fn json_call_to_bytes(json: &Json) -> Result<Vec<u8>, JsonError> {
    let wrapper: CallWrapper = serde_json::from_value(json.clone())
        .map_err(|e| JsonError::from_serde::<CallWrapper>(e))?;
    let account_addr = AccountAddr::new(wrapper.target.0);

    let tx = Transaction {
        version: wrapper.version,
        func_name: wrapper.func_name,
        target: account_addr,
        // verifydata,
        calldata: wrapper.calldata.0,
    };

    let mut buf = Vec::new();

    call::encode_call(&tx, &mut buf);

    Ok(buf)
}

#[derive(Debug, Serialize, Deserialize)]
struct WrappedCall {
    data: HexBlob<Vec<u8>>,
}

impl TypeInformation for WrappedCall {
    fn type_of_field_as_str(_field: &str) -> Option<&str> {
        Some("data")
    }
}

/// Given a binary [`Transaction`] wrapped inside JSON,
/// Decodes it and returns a user-friendly JSON.
///
/// ```json
/// {
///   "data": "E9E50C787F2076BD5E44"
/// }
/// ```
pub fn unwrap_binary_json_call(json: &Json) -> Result<Json, JsonError> {
    let wrapped_call: WrappedCall = serde_json::from_value(json.clone())
        .map_err(|e| JsonError::from_serde::<WrappedCall>(e))?;

    let tx = {
        let mut cursor = Cursor::new(&wrapped_call.data.0[..]);
        call::decode_call(&mut cursor).unwrap()
    };

    // let verifydata = json::bytes_to_str(&tx.verifydata);
    // let verifydata = json::decode_calldata(&json!({ "calldata": verifydata }))?;

    Ok(serde_json::to_value(CallWrapper {
        version: tx.version,
        target: AddressWrapper(tx.target.inner().clone()),
        func_name: tx.func_name.clone(),
        calldata: HexBlob(tx.calldata),
    })
    .unwrap())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::json;

    #[test]
    fn json_call_missing_version() {
        let json = json!({});

        let err = json_call_to_bytes(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "version".to_string(),
                reason: "value `null` isn\'t a(n) number".to_string(),
            }
        );
    }

    #[test]
    fn json_call_missing_target() {
        let json = json!({
            "version": 0
        });

        let err = json_call_to_bytes(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "target".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_call_missing_func_name() {
        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let err = json_call_to_bytes(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "func_name".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
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
        });

        let err = json_call_to_bytes(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "verifydata".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_call_missing_calldata() {
        let verifydata = json::encode_calldata(&json!({
            "abi": ["bool", "i8"],
            "data": [true, 3],
        }))
        .unwrap();

        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            "verifydata": verifydata["calldata"]
        });

        let err = json_call_to_bytes(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "calldata".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_call_valid() {
        let _verifydata = json::encode_calldata(&json!({
            "abi": ["bool", "i8"],
            "data": [true, 3],
        }))
        .unwrap();

        let calldata = json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20],
        }))
        .unwrap();

        let json = json!({
            "version": 0,
            "target": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            // "verifydata": verifydata["calldata"],
            "calldata": calldata["calldata"],
        });

        let bytes = json_call_to_bytes(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = unwrap_binary_json_call(&json!({ "data": data })).unwrap();

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
