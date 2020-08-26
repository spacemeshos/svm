use svm_nibble::{NibbleIter, NibbleWriter};

use serde_json::{json, Value};

use crate::{
    api::json::{self, JsonError},
    api::raw,
    transaction,
};

use svm_types::{AddressOf, App, AppTransaction, WasmValue};

///
/// ```json
/// {
///   version: 0,      // number
///   app: 'A2FB...',  // string
///   func: 'do_work', // string
///   calldata: '',    // string
/// }
/// ```
pub fn encode_exec_app(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")?;
    let app = json::as_addr(json, "app")?.into();
    let func = json::as_string(json, "func")?;

    let calldata = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&calldata, "calldata")?;

    let tx = AppTransaction {
        version,
        app,
        func,
        calldata,
    };

    let mut w = NibbleWriter::new();
    transaction::encode_exec_app(&tx, &mut w);

    let bytes = w.into_bytes();
    Ok(bytes)
}

pub fn decode_exec_app(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    let mut iter = NibbleIter::new(&bytes);
    let tx = raw::decode_exec_app(&mut iter).unwrap();

    let version = tx.version;
    let func = tx.func.clone();
    let app = json::addr_to_str(&tx.app.inner());

    let calldata = json::bytes_to_str(&tx.calldata);
    let calldata = json::decode_calldata(&json!({ "calldata": calldata }))?;

    let json = json!({
        "version": version,
        "app": app,
        "func": func,
        "calldata": calldata,
    });

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use svm_nibble::NibbleIter;
    use svm_types::{Address, WasmValue};

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
    fn json_exec_app_missing_func_index() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let err = encode_exec_app(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "func_index".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_exec_app_missing_calldata() {
        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_index": 0,
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
        let calldata = json::encode_calldata(&json!({
            "abi": ["i32", "i64"],
            "data": [10, 20],
        }))
        .unwrap();

        let json = json!({
            "version": 0,
            "app": "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
            "func_index": 1,
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
                "func_index": 1,
                "calldata": {
                    "abi": ["i32", "i64"],
                    "data": [10, 20]
                }
            })
        );
    }
}
