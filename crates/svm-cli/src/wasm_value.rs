use std::str::FromStr;

use svm_app::types::WasmValue;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    UnsupportedType(String),
}

/// Potentially converts `&str` to `WasmValue`.
/// Expected format is `{num}{type}` (`type` in (i32|i64)).
pub fn parse_str(value: &str) -> Result<WasmValue, ParseError> {
    let len = value.chars().count();
    let type_len = 3;

    if len < type_len + 1 {
        return Err(ParseError::InvalidFormat);
    }
    let mid = len - type_len;
    let num: String = value.chars().take(mid).collect();
    let num_t: String = value.chars().skip(mid).collect();

    match &num_t[..] {
        "i32" => match u32::from_str(&num) {
            Ok(v) => Ok(WasmValue::I32(v)),
            Err(_) => Err(ParseError::InvalidFormat),
        },
        "i64" => match u64::from_str(&num) {
            Ok(v) => Ok(WasmValue::I64(v)),
            Err(_) => Err(ParseError::InvalidFormat),
        },
        v => Err(ParseError::UnsupportedType(v.to_string())),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_str() {
        fn parse_str_ok(s: &str) -> WasmValue {
            parse_str(s).unwrap()
        }

        fn parse_str_err(s: &str) -> ParseError {
            parse_str(s).err().unwrap()
        }

        assert_eq!(parse_str_ok("0i32"), WasmValue::I32(0));
        assert_eq!(parse_str_ok("0i64"), WasmValue::I64(0));
        assert_eq!(parse_str_ok("1073741824i32"), WasmValue::I32(1073741824));
        assert_eq!(parse_str_ok("1073741824i64"), WasmValue::I64(1073741824));

        assert_eq!(parse_str_err(""), ParseError::InvalidFormat);
        assert_eq!(parse_str_err("0"), ParseError::InvalidFormat);
        assert_eq!(parse_str_err("i32"), ParseError::InvalidFormat);
        assert_eq!(parse_str_err("i64"), ParseError::InvalidFormat);

        assert_eq!(
            parse_str_err("10i6"),
            ParseError::UnsupportedType(String::from("0i6"))
        );
        assert_eq!(
            parse_str_err("10i63"),
            ParseError::UnsupportedType(String::from("i63"))
        );
        assert_eq!(
            parse_str_err("10i633"),
            ParseError::UnsupportedType(String::from("633"))
        );
    }
}
