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
        assert_eq!(parse_str("0i32").unwrap(), WasmValue::I32(0));
        assert_eq!(parse_str("0i64").unwrap(), WasmValue::I64(0));
        assert_eq!(
            parse_str("1073741824i32").unwrap(),
            WasmValue::I32(1073741824)
        );
        assert_eq!(
            parse_str("1073741824i64").unwrap(),
            WasmValue::I64(1073741824)
        );

        assert_eq!(parse_str("").err().unwrap(), ParseError::InvalidFormat);
        assert_eq!(parse_str("0").err().unwrap(), ParseError::InvalidFormat);
        assert_eq!(parse_str("i32").err().unwrap(), ParseError::InvalidFormat);
        assert_eq!(parse_str("i64").err().unwrap(), ParseError::InvalidFormat);

        assert_eq!(
            parse_str("10i6").err().unwrap(),
            ParseError::UnsupportedType(String::from("0i6"))
        );
        assert_eq!(
            parse_str("10i63").err().unwrap(),
            ParseError::UnsupportedType(String::from("i63"))
        );
        assert_eq!(
            parse_str("10i633").err().unwrap(),
            ParseError::UnsupportedType(String::from("633"))
        );
    }
}
