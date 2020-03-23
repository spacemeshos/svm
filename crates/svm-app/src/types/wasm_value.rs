use crate::types::wasm_value::WasmValueError::InvalidFormat;
use std::convert::TryFrom;
use std::str::FromStr;

/// Wasm Integer.
#[derive(Clone, PartialEq, Debug)]
pub enum WasmValue {
    /// A 32-bit integer.
    I32(u32),

    /// A 64-bit integer.
    I64(u64),
}

/// Wasm function argument error.
#[derive(Debug)]
pub enum WasmValueError {
    /// Invalid Format.
    InvalidFormat,

    /// Unsupported type.
    UnsupportedType(String),
}

const TYPE_CHARS_LEN: usize = 3;

/// Potentially converts `&str` to `WasmValue`.
/// TODO: specify format
impl TryFrom<&str> for WasmValue {
    type Error = WasmValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if len < TYPE_CHARS_LEN + 1 {
            return Err(InvalidFormat);
        }
        let mid = len - TYPE_CHARS_LEN;
        let num: String = value.chars().take(mid).collect();
        let num_t: String = value.chars().skip(mid).collect();

        match &num_t[..] {
            "i32" => {
                match u32::from_str(&num) {
                    Ok(v) => return Ok(WasmValue::I32(v)),
                    Err(_) => return Err(InvalidFormat),
                };
            }
            "i64" => {
                match u64::from_str(&num) {
                    Ok(v) => return Ok(WasmValue::I64(v)),
                    Err(_) => return Err(InvalidFormat),
                };
            }
            v => Err(WasmValueError::UnsupportedType(v.to_string())),
        }
    }
}
