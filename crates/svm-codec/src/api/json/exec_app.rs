use std::io::Cursor;

use serde_json::{json, Value};

use crate::api::json::{self, JsonError};
use crate::transaction;

use svm_types::Transaction;

///
/// ```json
/// {
///   version: 0,           // number
///   app: 'A2FB...',       // string
///   func_name: 'do_work', // string
///   verifydata: '',       // string
///   calldata: '',         // string
/// }
/// ```
pub fn encode_exec_app(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")? as u16;
    let app = json::as_addr(json, "app")?.into();
    let func_name = json::as_string(json, "func_name")?;

    let verifydata = json::as_string(json, "verifydata")?;
    let verifydata = json::str_to_bytes(&verifydata, "verifydata")?;

    let calldata = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&calldata, "calldata")?;

    let tx = Transaction {
        version,
        app,
        func_name,
        verifydata,
        calldata,
    };

    let mut buf = Vec::new();

    transaction::encode_exec_app(&tx, &mut buf);

    Ok(buf)
}

/// Given a binary `exec-app` transaction wrapped inside JSON.
/// Decodes it and returns a user-friendly JSON.
pub fn decode_exec_app(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    let mut cursor = Cursor::new(&bytes[..]);
    let tx = transaction::decode_exec_app(&mut cursor).unwrap();

    let version = tx.version;
    let func_name = tx.func_name.clone();
    let app = json::addr_to_str(&tx.app.inner());

    let verifydata = json::bytes_to_str(&tx.verifydata);
    let verifydata = json::decode_calldata(&json!({ "calldata": verifydata }))?;

    let calldata = json::bytes_to_str(&tx.calldata);
    let calldata = json::decode_calldata(&json!({ "calldata": calldata }))?;

    let json = json!({
        "version": version,
        "app": app,
        "func_name": func_name,
        "verifydata": verifydata,
        "calldata": calldata,
    });

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn json_exec_app_missing_version() {
        let json = json!({});

        let err = encode_exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "version".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_app_addr() {
        let json = json!({
            "version": 0
        });

        let err = encode_exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "app".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_func_name() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let err = encode_exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "func_name".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_verifydata() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
        });

        let err = encode_exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "verifydata".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_calldata() {
        let verifydata = json::encode_calldata(&json!({
            "abi": ["bool", "i8"],
            "data": [true, 3],
        }))
        .unwrap();

        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            "verifydata": verifydata["calldata"]
        });

        let err = encode_exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "calldata".to_string(),
                reason: "value `null` isn\'t a string".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_valid() {
        let verifydata = json::encode_calldata(&json!({
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
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_name": "do_something",
            "verifydata": verifydata["calldata"],
            "calldata": calldata["calldata"],
        });

        let bytes = encode_exec_app(&json).unwrap();
        let data = json::bytes_to_str(&bytes);
        let json = decode_exec_app(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "version": 0,
                "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
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
