mod calldata;
mod deploy_template;
mod error;
mod exec_app;
mod func_args;
mod func_buf;
mod spawn_app;

pub use calldata::{decode_calldata, encode_calldata};
pub use deploy_template::deploy_template;
pub use error::JsonError;
pub use exec_app::{decode_exec_app, exec_app};
pub use func_args::{decode_func_args, encode_func_args};
pub use func_buf::{decode_func_buf, encode_func_buf};
pub use spawn_app::{decode_spawn_app, spawn_app};

use serde_json::Value;

use svm_types::{Address, WasmValue};

pub(crate) fn to_bytes(json: &Value) -> Result<Vec<u8>, JsonError> {
    match serde_json::to_string(&json) {
        Ok(s) => Ok(s.into_bytes()),
        Err(e) => Err(JsonError::Unknown(format!("{}", e))),
    }
}

pub(crate) fn as_array<'a>(json: &'a Value, field: &str) -> Result<&'a Vec<Value>, JsonError> {
    let value: &Value = &json[field];

    match value.as_array() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't an Array", value),
        }),
        Some(value) => Ok(value),
    }
}

pub(crate) fn as_u16(json: &Value, field: &str) -> Result<u16, JsonError> {
    let value: &Value = &json[field];

    match value.as_u64() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", value),
        }),
        Some(value) => {
            if value > std::u16::MAX as u64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into 16-bit integer", value),
                })
            } else {
                Ok(value as u16)
            }
        }
    }
}

pub(crate) fn as_u32(json: &Value, field: &str) -> Result<u32, JsonError> {
    let value: &Value = &json[field];

    match value.as_u64() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a number", value),
        }),
        Some(value) => {
            if value > std::u32::MAX as u64 {
                Err(JsonError::InvalidField {
                    field: field.to_string(),
                    reason: format!("value `{}` doesn't fit into 32-bit integer", value),
                })
            } else {
                Ok(value as u32)
            }
        }
    }
}

fn as_byte(c1: char, c2: char) -> u8 {
    let c1 = c1.to_digit(16).unwrap() as u8;
    let c2 = c2.to_digit(16).unwrap() as u8;

    (c1 << 4) | c2
}

pub(crate) fn as_string(json: &Value, field: &str) -> Result<String, JsonError> {
    let value: &Value = &json[field];

    match value.as_str() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a string", value),
        }),
        Some(value) => Ok(value.to_string()),
    }
}

pub(crate) fn bytes_to_str(bytes: &[u8]) -> String {
    svm_common::fmt::fmt_hex(bytes, "")
}

pub(crate) fn str_to_bytes(value: &str, field: &str) -> Result<Vec<u8>, JsonError> {
    if value.len() % 2 == 1 {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should be of even length".to_string(),
        });
    }

    if value.chars().any(|c| c.is_ascii_hexdigit() == false) {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should have only hex digits".to_string(),
        });
    }

    let bytes = value
        .chars()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|slice| {
            let (c1, c2) = (slice[0], slice[1]);
            as_byte(c1, c2)
        })
        .collect();

    Ok(bytes)
}

pub(crate) fn as_blob(json: &Value, field: &str) -> Result<Vec<u8>, JsonError> {
    let value = as_string(json, field)?;
    str_to_bytes(&value, field)
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

pub(crate) fn as_wasm_value(json: &Value, field: &str) -> Result<WasmValue, JsonError> {
    let value = match json {
        Value::String(s) => s,
        _ => {
            return Err(JsonError::InvalidField {
                field: field.to_string(),
                reason: format!("wasm vaulue should be of a string"),
            })
        }
    };

    let len = value.len();
    let is_i32 = value.ends_with("i32");
    let is_i64 = value.ends_with("i64");
    let mut valid = true;

    if valid && is_i32 {
        let value = &value[0..(len - 3)];

        match value.parse::<u32>() {
            Ok(v) => return Ok(WasmValue::I32(v)),
            Err(..) => valid = false,
        }
    }

    if valid && is_i64 {
        let value = &value[0..(len - 3)];

        match value.parse::<u64>() {
            Ok(v) => return Ok(WasmValue::I64(v)),
            Err(..) => valid = false,
        }
    }

    debug_assert!(!valid);

    Err(JsonError::InvalidField {
        field: field.to_string(),
        reason: "item should be of pattern `{number}i32` or `{number}i64`".to_string(),
    })
}

pub(crate) fn as_wasm_values(json: &Value, field: &str) -> Result<Vec<WasmValue>, JsonError> {
    let value: &Value = &json[field];

    match value.as_array() {
        Some(vec) => {
            let mut values = Vec::with_capacity(vec.len());
            let field = format!("{} (array item)", field);

            for v in vec.iter() {
                let v = as_wasm_value(v, &field)?;
                values.push(v);
            }

            Ok(values)
        }
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't an array", value),
        }),
    }
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
    fn json_as_wasm_values_i32_valid() {
        let json = json!({
            "args": ["10i32", "20i32"]
        });

        let args = as_wasm_values(&json, "args").unwrap();
        assert_eq!(args, vec![WasmValue::I32(10), WasmValue::I32(20)]);
    }

    #[test]
    fn json_as_wasm_values_i64_valid() {
        let json = json!({
            "args": ["10i64", "20i64"]
        });

        let args = as_wasm_values(&json, "args").unwrap();
        assert_eq!(args, vec![WasmValue::I64(10), WasmValue::I64(20)]);
    }

    #[test]
    fn json_as_wasm_values_i32_and_i64_valid() {
        let json = json!({
            "args": ["10i32", "20i64"]
        });

        let args = as_wasm_values(&json, "args").unwrap();
        assert_eq!(args, vec![WasmValue::I32(10), WasmValue::I64(20)]);
    }

    #[test]
    fn json_as_wasm_values_i32_invalid() {
        let json = json!({
            "args": ["NaNi32"]
        });

        let err = as_wasm_values(&json, "args").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "args (array item)".to_string(),
                reason: "item should be of pattern `{number}i32` or `{number}i64`".to_string(),
            }
        );
    }

    #[test]
    fn json_as_wasm_values_i64_invalid() {
        let json = json!({
            "args": ["NaNi64"]
        });

        let err = as_wasm_values(&json, "args").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "args (array item)".to_string(),
                reason: "item should be of pattern `{number}i32` or `{number}i64`".to_string(),
            }
        );
    }

    #[test]
    fn json_as_wasm_values_invalid_type() {
        let json = json!({
            "args": "10i32"
        });

        let err = as_wasm_values(&json, "args").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "args".to_string(),
                reason: r#"value `"10i32"` isn't an array"#.to_string()
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
