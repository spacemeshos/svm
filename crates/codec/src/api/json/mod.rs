mod calldata;
mod deploy_template;
mod error;
mod exec_app;
mod receipt;
mod spawn_app;

pub use calldata::{decode_calldata, encode_calldata};
pub use deploy_template::deploy_template;
pub use error::JsonError;
pub use exec_app::{decode_exec_app, encode_exec_app};
pub use receipt::decode_receipt;
pub use spawn_app::{decode_spawn_app, encode_spawn_app};

use serde_json::{json, Value};

use svm_sdk_types::Amount;
use svm_types::{Address, Gas, ReceiptLog, State};

pub(crate) fn to_bytes(json: &Value) -> Result<Vec<u8>, JsonError> {
    match serde_json::to_string(&json) {
        Ok(s) => Ok(s.into_bytes()),
        Err(e) => Err(JsonError::Unknown(format!("{}", e))),
    }
}

pub(crate) fn as_bool(json: &Value, field: &str) -> Result<bool, JsonError> {
    let v: &Value = &json[field];

    v.as_bool().ok_or(JsonError::InvalidField {
        field: field.to_string(),
        reason: format!("value `{}` isn't a boolean", v),
    })
}

pub(crate) fn as_u8(json: &Value, field: &str) -> Result<u8, JsonError> {
    let v: &Value = &json[field];

    v.as_u64()
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        })
        .and_then(|v| {
            if v > std::u8::MAX as u64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into `u8`", v),
                })
            } else {
                Ok(v as u8)
            }
        })
}

pub(crate) fn as_i8(json: &Value, field: &str) -> Result<i8, JsonError> {
    let v: &Value = &json[field];

    v.as_i64()
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        })
        .and_then(|v| {
            if v > std::i8::MAX as i64 || v < std::i8::MIN as i64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into `i8`", v),
                })
            } else {
                Ok(v as i8)
            }
        })
}

pub(crate) fn as_u16(json: &Value, field: &str) -> Result<u16, JsonError> {
    let v: &Value = &json[field];

    v.as_u64()
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        })
        .and_then(|v| {
            if v > std::u16::MAX as u64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into `u16` integer", v),
                })
            } else {
                Ok(v as u16)
            }
        })
}

pub(crate) fn as_i16(json: &Value, field: &str) -> Result<i16, JsonError> {
    let v: &Value = &json[field];

    match v.as_i64() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        }),
        Some(v) => {
            if v > std::i16::MAX as i64 || v < std::i16::MIN as i64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into `i16` integer", v),
                })
            } else {
                Ok(v as i16)
            }
        }
    }
}

pub(crate) fn as_u32(json: &Value, field: &str) -> Result<u32, JsonError> {
    let v: &Value = &json[field];

    v.as_u64()
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        })
        .and_then(|v| {
            if v > std::u32::MAX as u64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into `u32` integer", v),
                })
            } else {
                Ok(v as u32)
            }
        })
}

pub(crate) fn as_i32(json: &Value, field: &str) -> Result<i32, JsonError> {
    let v: &Value = &json[field];

    v.as_i64()
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        })
        .and_then(|v| {
            if v > std::i32::MAX as i64 || v < std::i32::MIN as i64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into `i32` integer", v),
                })
            } else {
                Ok(v as i32)
            }
        })
}

pub(crate) fn as_u64(json: &Value, field: &str) -> Result<u64, JsonError> {
    let v: &Value = &json[field];

    v.as_u64().ok_or(JsonError::InvalidField {
        field: field.to_string(),
        reason: format!("value `{}` isn't a u64 number", v),
    })
}

pub(crate) fn as_i64(json: &Value, field: &str) -> Result<i64, JsonError> {
    let v: &Value = &json[field];

    v.as_i64().ok_or(JsonError::InvalidField {
        field: field.to_string(),
        reason: format!("value `{}` isn't a i64 number", v),
    })
}

pub(crate) fn as_amount(json: &Value, field: &str) -> Result<Amount, JsonError> {
    let v: &Value = &json[field];

    v.as_u64()
        .map(|v| Amount(v))
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", v),
        })
}

pub(crate) fn as_string(json: &Value, field: &str) -> Result<String, JsonError> {
    let v: &Value = &json[field];

    v.as_str()
        .map(|v| v.to_string())
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a string", v),
        })
}

pub(crate) fn as_array<'a>(json: &'a Value, field: &str) -> Result<&'a Vec<Value>, JsonError> {
    let v: &Value = &json[field];

    v.as_array().ok_or(JsonError::InvalidField {
        field: field.to_string(),
        reason: format!("value `{}` isn't an Array", v),
    })
}

pub(crate) fn bytes_to_str(bytes: &[u8]) -> String {
    svm_common::fmt::fmt_hex(bytes, "")
}

pub(crate) fn str_to_bytes(v: &str, field: &str) -> Result<Vec<u8>, JsonError> {
    if v.len() % 2 == 1 {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should be of even length".to_string(),
        });
    }

    if v.chars().any(|c| c.is_ascii_hexdigit() == false) {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should have only hex digits".to_string(),
        });
    }

    fn chars_as_byte(c1: char, c2: char) -> u8 {
        let c1 = c1.to_digit(16).unwrap() as u8;
        let c2 = c2.to_digit(16).unwrap() as u8;

        (c1 << 4) | c2
    }

    let bytes = v
        .chars()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|slice| {
            let (c1, c2) = (slice[0], slice[1]);
            chars_as_byte(c1, c2)
        })
        .collect();

    Ok(bytes)
}

pub(crate) fn as_blob(json: &Value, field: &str) -> Result<Vec<u8>, JsonError> {
    let v = as_string(json, field)?;
    str_to_bytes(&v, field)
}

pub(crate) fn as_addr(json: &Value, field: &str) -> Result<Address, JsonError> {
    let s = as_string(json, field)?;
    str_as_addr(&s, field)
}

pub(crate) fn str_as_addr(s: &str, field: &str) -> Result<Address, JsonError> {
    let bytes = str_to_bytes(s, field)?;

    if bytes.len() != Address::len() {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value should be exactly {} hex digits", Address::len() * 2),
        });
    }

    let addr: Address = (&bytes[..]).into();
    Ok(addr)
}

pub(crate) fn addr_to_str(addr: &Address) -> String {
    bytes_to_str(addr.as_slice())
}

pub(crate) fn state_to_str(state: &State) -> String {
    bytes_to_str(state.as_slice())
}

pub(crate) fn gas_to_json(gas: &Gas) -> i64 {
    if gas.is_some() {
        gas.unwrap() as _
    } else {
        -1
    }
}

pub(crate) fn logs_to_json(logs: &[ReceiptLog]) -> Vec<Value> {
    logs.iter()
        .map(|log| {
            let msg = unsafe { String::from_utf8_unchecked(log.msg.clone()) };

            json!({
                "msg": msg,
                "code": log.code
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

    #[test]
    fn json_as_u16_valid() {
        let json = json!({
            "n": 10
        });

        let n = as_u16(&json, "n").unwrap();
        assert_eq!(n, 10u16);
    }

    #[test]
    fn json_as_u16_invalid_field() {
        let json = json!({
            "n": "NaN"
        });

        let err = as_u16(&json, "n").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "n".to_string(),
                reason: r#"value `"NaN"` isn't a number"#.to_string()
            }
        );
    }

    #[test]
    fn json_as_u32_valid() {
        let json = json!({
            "n": 10
        });

        let n = as_u32(&json, "n").unwrap();
        assert_eq!(n, 10u32);
    }

    #[test]
    fn json_as_u32_invalid_field() {
        let json = json!({
            "n": "NaN"
        });

        let err = as_u32(&json, "n").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "n".to_string(),
                reason: r#"value `"NaN"` isn't a number"#.to_string()
            }
        );
    }

    #[test]
    fn json_as_address_valid() {
        let json = json!({
            "addr": "10203040506070809000A0B0C0D0E0F0ABCDEFFF"
        });

        let addr = as_addr(&json, "addr").unwrap();
        let actual = addr.bytes();

        let expected = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0x00, 0xA0, 0xB0, 0xC0, 0xD0,
            0xE0, 0xF0, 0xAB, 0xCD, 0xEF, 0xFF,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn json_as_address_invalid_type() {
        let json = json!({
            "addr": true
        });

        let err = as_addr(&json, "addr").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "addr".to_string(),
                reason: r#"value `true` isn't a string"#.to_string()
            }
        );
    }

    #[test]
    fn json_as_address_invalid_length() {
        let json = json!({
            "addr": "1020"
        });

        let err = as_addr(&json, "addr").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "addr".to_string(),
                reason: "value should be exactly 40 hex digits".to_string(),
            }
        );
    }

    #[test]
    fn json_as_address_invalid_chars() {
        let json = json!({
            "addr": "XYWZ"
        });

        let err = as_addr(&json, "addr").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "addr".to_string(),
                reason: "value should have only hex digits".to_string(),
            }
        );
    }

    #[test]
    fn json_as_blob_valid() {
        let json = json!({
            "blob": "1DB30F"
        });

        let blob = as_blob(&json, "blob").unwrap();
        assert_eq!(blob, vec![0x1D, 0xB3, 0x0F])
    }

    #[test]
    fn json_as_blob_invalid_chars() {
        let json = json!({
            "blob": "NOT HEX!"
        });

        let err = as_blob(&json, "blob").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "blob".to_string(),
                reason: "value should have only hex digits".to_string(),
            }
        );
    }

    #[test]
    fn json_as_blob_invalid_odd_length() {
        let json = json!({
            "blob": "A0B"
        });

        let err = as_blob(&json, "blob").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "blob".to_string(),
                reason: "value should be of even length".to_string(),
            }
        );
    }
}
