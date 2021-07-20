use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use std::io::Cursor;

use svm_types::{AccountAddr, Address, Transaction};

use super::{HexBlob, TypeInformation};
use crate::api::json::{self, JsonError};
use crate::call;

#[derive(Clone, Serialize, Deserialize)]
struct CallJsonlike {
    version: u16,
    target: HexBlob,
    func_name: String,
    // verifydata: String,
    calldata: HexBlob,
}

impl CallJsonlike {
    fn account_addr(&self) -> AccountAddr {
        AccountAddr::new(Address::from(&self.target.0[..]))
    }
}

impl TypeInformation for CallJsonlike {
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
///   version: 0,           // number
///   target: 'A2FB...',   // string
///   func_name: 'do_work', // string
///   verifydata: '',       // string
///   calldata: '',         // string
/// }
/// ```
pub fn encode_call(json: &Value) -> Result<Vec<u8>, JsonError> {
    let jsonlike: CallJsonlike = serde_json::from_value(json.clone())
        .map_err(|e| JsonError::from_serde::<CallJsonlike>(e))?;
    let account_addr = jsonlike.account_addr();

    let tx = Transaction {
        version: jsonlike.version,
        func_name: jsonlike.func_name,
        target: account_addr,
        // verifydata,
        calldata: jsonlike.calldata.0,
    };

    let mut buf = Vec::new();

    call::encode_call(&tx, &mut buf);

    Ok(buf)
}

#[derive(Serialize, Deserialize)]
struct WrappedCall {
    data: HexBlob,
}

/// Given a binary [`Transaction`] wrapped inside JSON,
/// Decodes it and returns a user-friendly JSON.
pub fn decode_call(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    let mut cursor = Cursor::new(&bytes[..]);
    let tx = call::decode_call(&mut cursor).unwrap();

    let version = tx.version;
    let func_name = tx.func_name.clone();
    let target = json::addr_to_str(&tx.target.inner());

    // let verifydata = json::bytes_to_str(&tx.verifydata);
    // let verifydata = json::decode_calldata(&json!({ "calldata": verifydata }))?;

    let calldata = json::bytes_to_str(&tx.calldata);
    let calldata = json::decode_calldata(&json!({ "calldata": calldata }))?;

    let json = json!({
        "version": version,
        "target": target,
        "func_name": func_name,
        // "verifydata": verifydata,
        "calldata": calldata,
    });

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn json_call_missing_version() {
        let json = json!({});

        let err = encode_call(&json).unwrap_err();
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

        let err = encode_call(&json).unwrap_err();
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

        let err = encode_call(&json).unwrap_err();
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

        let err = encode_call(&json).unwrap_err();
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

        let err = encode_call(&json).unwrap_err();
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

        let bytes = encode_call(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = decode_call(&json!({ "data": data })).unwrap();

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
