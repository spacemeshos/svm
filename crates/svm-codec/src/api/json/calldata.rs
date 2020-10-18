use serde_json::json;
use serde_json::Value as Json;

use svm_abi_decoder::{Cursor, Decoder};
use svm_abi_encoder::Encoder;
use svm_sdk::value::{Composite, Primitive, Value};
use svm_sdk::Address;

use crate::api::json::{self, JsonError};
use crate::api::raw;

macro_rules! as_str {
    ($json:expr) => {{
        let s = $json.as_str();

        if s.is_none() {
            return Err(JsonError::InvalidField {
                field: "data".to_string(),
                reason: "non-string value".to_string(),
            });
        }

        Ok(s.unwrap())
    }};
}

pub fn encode_calldata(json: &Json) -> Result<Json, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    if abi.len() != data.len() {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "`abi` and `data` must be of the same length".to_string(),
        });
    }

    let mut buf = Vec::new();

    let nargs = abi.len() as u8;
    buf.push(nargs);

    for (ty, raw) in abi.iter().zip(data) {
        let value = encode_value(ty, raw)?;
        value.encode(&mut buf);
    }

    let calldata = json::bytes_to_str(&buf);
    let json = json!({ "calldata": calldata });

    Ok(json)
}

pub fn decode_calldata(json: &Json) -> Result<Json, JsonError> {
    let data = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&data, "calldata")?;

    let nargs = calldata[0];

    let mut decoder = Decoder::new();
    let mut cursor = Cursor::new(&calldata[1..]);

    let mut abi: Vec<Json> = Vec::new();
    let mut data: Vec<Json> = Vec::new();

    for _ in 0..nargs {
        let value: Value = decoder.decode_value(&mut cursor).unwrap();
        let (ty, item) = value_as_json(&value);

        abi.push(ty);
        data.push(item);
    }

    let result = json!({ "abi": abi, "data": data });
    Ok(result)
}

fn value_as_json(value: &Value) -> (Json, Json) {
    match value {
        Value::Primitive(p) => primitive_as_json(p),
        Value::Composite(c) => composite_as_json(c),
    }
}

fn primitive_as_json(p: &Primitive) -> (Json, Json) {
    match p {
        Primitive::Bool(b) => (Json::String("bool".into()), json!(b)),
        Primitive::Amount(a) => (Json::String("amount".into()), json!(a.0)),
        Primitive::I8(n) => (Json::String("i8".into()), json!(n)),
        Primitive::U8(n) => (Json::String("u8".into()), json!(n)),
        Primitive::I16(n) => (Json::String("i16".into()), json!(n)),
        Primitive::U16(n) => (Json::String("u16".into()), json!(n)),
        Primitive::I32(n) => (Json::String("i32".into()), json!(n)),
        Primitive::U32(n) => (Json::String("u32".into()), json!(n)),
        Primitive::I64(n) => (Json::String("i64".into()), json!(n)),
        Primitive::U64(n) => (Json::String("u64".into()), json!(n)),
        Primitive::Address(addr) => {
            let s = json::bytes_to_str(addr.as_slice());
            (Json::String("address".into()), json!(s))
        }
        Primitive::None => unreachable!(),
    }
}

fn composite_as_json(c: &Composite<'_>) -> (Json, Json) {
    let array: &[Value] = match c {
        Composite::Array(inner) => inner,
        Composite::ArrayOwned(inner) => inner,
    };

    if (array.is_empty()) {
        return (Json::Null, Json::Array(Vec::new()));
    }

    let mut types: Vec<Json> = Vec::new();
    let mut values: Vec<Json> = Vec::new();

    for elem in array {
        let (ty, value) = value_as_json(elem);

        types.push(ty);
        values.push(value);
    }

    // TODO: assert that all `types` are the same
    let ty = types.pop().unwrap();

    (Json::Array(vec![ty]), Json::Array(values))
}

fn encode_value<'a>(ty: &Json, value: &Json) -> Result<Value<'static>, JsonError> {
    if ty.is_array() {
        return encode_array(ty, value);
    }

    let ty = as_str!(ty)?;
    let json = json!({ "calldata": value });

    macro_rules! encode {
        ($func:ident) => {{
            json::$func(&json, "calldata")?.into()
        }};
    }

    let value: Value = match ty {
        "bool" => encode!(as_bool),
        "i8" => encode!(as_i8),
        "u8" => encode!(as_u8),
        "i16" => encode!(as_i16),
        "u16" => encode!(as_u16),
        "i32" => encode!(as_i32),
        "u32" => encode!(as_u32),
        "i64" => encode!(as_i64),
        "u64" => encode!(as_u64),
        "amount" => encode!(as_amount),
        "address" => {
            let addr: svm_types::Address = json::as_addr(&json, "calldata")?;

            let bytes = addr.bytes();
            let addr: Address = bytes.into();

            addr.into()
        }
        _ => {
            return Err(JsonError::InvalidField {
                field: "abi".to_string(),
                reason: format!("invalid ABI type: `{}`", ty),
            })
        }
    };

    Ok(value)
}

fn encode_array(ty: &Json, value: &Json) -> Result<Value<'static>, JsonError> {
    debug_assert!(ty.is_array());

    let types = ty.as_array().unwrap();
    assert_eq!(types.len(), 1);

    let ty = &types[0];

    let json = json!({ "calldata": value });
    let elems = json::as_array(&json, "calldata")?;

    let mut array = Vec::new();

    for elem in elems {
        let elem = encode_value(ty, elem)?;
        array.push(elem);
    }

    let c = Composite::ArrayOwned(array);
    Ok(Value::Composite(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($abi:expr, $data:expr) => {{
            let json = json!({"abi": $abi, "data": $data });

            let encoded = encode_calldata(&json).unwrap();
            let decoded = decode_calldata(&encoded).unwrap();

            assert_eq!(
                decoded,
                json!({"abi": $abi, "data": $data })
            );
        }}
    }

    #[test]
    pub fn encode_calldata_bool() {
        test!(["bool", "bool"], [true, false]);
    }

    #[test]
    pub fn encode_calldata_i8_u8() {
        test!(["i8", "u8"], [std::i8::MIN as isize, std::u8::MAX as isize]);
    }

    #[test]
    pub fn encode_calldata_i16_u16() {
        test!(
            ["i16", "u16"],
            [std::i16::MIN as isize, std::u16::MAX as isize]
        );
    }

    #[test]
    pub fn encode_calldata_i32_u32() {
        test!(
            ["i32", "u32"],
            [std::i32::MIN as isize, std::u32::MAX as isize]
        );
    }

    #[test]
    pub fn encode_calldata_i64_u64() {
        test!(["i64"], [std::i64::MIN as isize]);
        test!(["u64"], [std::u64::MAX as usize]);
    }

    #[test]
    pub fn encode_calldata_amount() {
        test!(["amount", "amount"], [10 as u64, 20 as u64]);
    }

    #[test]
    pub fn encode_calldata_address() {
        let addr = "1020304050607080900010203040506070809000";

        test!(["address"], [addr]);
    }

    #[test]
    pub fn encode_calldata_array() {
        test!([["u32"]], [[10, 20, 30]]);
        test!([["i8"]], [[-10, 0, 30]]);
        test!([["u32"], ["i8"]], [[10, 20, 30], [-10, 0, 20]]);
    }
}
