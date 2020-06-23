mod deploy_template;
mod error;
mod exec_app;
mod spawn_app;

pub use deploy_template::deploy_template;
pub use error::JsonError;
pub use exec_app::exec_app;
pub use spawn_app::spawn_app;

use serde_json::Value;

use svm_common::Address;
use svm_types::WasmValue;

fn as_u16(json: &Value, field: &str) -> Result<u16, JsonError> {
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

fn as_u32(json: &Value, field: &str) -> Result<u32, JsonError> {
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

fn as_string(json: &Value, field: &str) -> Result<String, JsonError> {
    let value: &Value = &json[field];

    match value.as_str() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a string", value),
        }),
        Some(value) => Ok(value.to_string()),
    }
}

fn as_addr(json: &Value, field: &str) -> Result<Address, JsonError> {
    let value = as_string(json, field)?;

    if value.chars().any(|c| c.is_ascii_hexdigit() == false) {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should have only {} hex digits".to_string(),
        });
    }

    if value.len() != Address::len() * 2 {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should be exactly {} hex digits".to_string(),
        });
    }

    let chars: Vec<char> = value.chars().collect();
    let bytes: Vec<u8> = chars
        .as_slice()
        .chunks_exact(2)
        .map(|slice| {
            let (c1, c2) = (slice[0], slice[1]);
            as_byte(c1, c2)
        })
        .collect();

    debug_assert_eq!(bytes.len(), Address::len());

    let addr = Address::from(&bytes[..]);
    Ok(addr)
}

fn as_wasm_value(json: &Value, field: &str) -> Result<WasmValue, JsonError> {
    let value = json.as_str().unwrap();
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

fn as_wasm_values(json: &Value, field: &str) -> Result<Vec<WasmValue>, JsonError> {
    let value: &Value = &json[field];

    match value.as_array() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't an array", value),
        }),
        Some(vec) => {
            let mut values = Vec::with_capacity(vec.len());
            let field = format!("{} (array item)", field);

            for v in vec.iter() {
                let v = as_wasm_value(v, &field)?;
                values.push(v);
            }

            Ok(values)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::Value;

    #[test]
    fn json_as_u16_valid() {
        let data = r#"{ "n": 10 }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let n = as_u16(&v, "n").unwrap();

        assert_eq!(n, 10u16);
    }

    #[test]
    fn json_as_u16_invalid_field() {
        let data = r#"{ "n": "NaN" }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_u16(&v, "n").unwrap_err();

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
        let data = r#"{ "n": 10 }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let n = as_u32(&v, "n").unwrap();

        assert_eq!(n, 10u32);
    }

    #[test]
    fn json_as_u32_invalid_field() {
        let data = r#"{ "n": "NaN" }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_u32(&v, "n").unwrap_err();

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
        let data = r#"{ "addr": "10203040506070809000A0B0C0D0E0F0ABCDEFFF" }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let addr: Address = as_addr(&v, "addr").unwrap();

        let actual = addr.bytes();

        let expected = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0x00, 0xA0, 0xB0, 0xC0, 0xD0,
            0xE0, 0xF0, 0xAB, 0xCD, 0xEF, 0xFF,
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn json_as_address_invalid_type() {
        let data = r#"{ "addr": true }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_addr(&v, "addr").unwrap_err();

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
        let data = r#"{ "addr": "1020" }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_addr(&v, "addr").unwrap_err();

        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "addr".to_string(),
                reason: "value should be exactly {} hex digits".to_string(),
            }
        );
    }

    #[test]
    fn json_as_address_invalid_chars() {
        let data = r#"{ "addr": "XYZ" }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_addr(&v, "addr").unwrap_err();

        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "addr".to_string(),
                reason: "value should have only {} hex digits".to_string(),
            }
        );
    }

    #[test]
    fn json_as_wasm_values_i32_valid() {
        let data = r#"{ "args": ["10i32", "20i32"] }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let args = as_wasm_values(&v, "args").unwrap();

        assert_eq!(args, vec![WasmValue::I32(10), WasmValue::I32(20)]);
    }

    #[test]
    fn json_as_wasm_values_i64_valid() {
        let data = r#"{ "args": ["10i64", "20i64"] }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let args = as_wasm_values(&v, "args").unwrap();

        assert_eq!(args, vec![WasmValue::I64(10), WasmValue::I64(20)]);
    }

    #[test]
    fn json_as_wasm_values_i32_and_i64_valid() {
        let data = r#"{ "args": ["10i32", "20i64"] }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let args = as_wasm_values(&v, "args").unwrap();

        assert_eq!(args, vec![WasmValue::I32(10), WasmValue::I64(20)]);
    }

    #[test]
    fn json_as_wasm_values_i32_invalid() {
        let data = r#"{ "args": ["NaNi32"] }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_wasm_values(&v, "args").unwrap_err();

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
        let data = r#"{ "args": ["NaNi64"] }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_wasm_values(&v, "args").unwrap_err();

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
        let data = r#"{ "args": "10i32" }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let err = as_wasm_values(&v, "args").unwrap_err();

        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "args".to_string(),
                reason: r#"value `"10i32"` isn't an array"#.to_string()
            }
        );
    }
}
