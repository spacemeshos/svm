use serde_json::json;
use serde_json::Value as Json;

use svm_abi_decoder::CallData;
use svm_abi_encoder::{ByteSize, Encoder};
use svm_sdk_types::value::{Composite, Primitive, Value};
use svm_sdk_types::{Address, Amount};

use crate::api::json::{self, JsonError};

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

/// Given a `Calldata` JSON, encodes it into a binary `Calldata`
/// and returns the result wrapped with a JSON
pub fn encode_calldata(json: &Json) -> Result<Json, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    if abi.len() != data.len() {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "`abi` and `data` must be of the same length".to_string(),
        });
    }

    let mut cap = 0;

    for (ty, raw) in abi.iter().zip(data) {
        cap += value_byte_size(ty, raw)?;
    }

    let mut buf = svm_sdk_std::Vec::with_capacity(cap);

    for (ty, raw) in abi.iter().zip(data) {
        let value = encode_value(ty, raw)?;

        value.encode(&mut buf);
    }

    let calldata = json::bytes_to_str(buf.as_slice());
    let json = json!({ "calldata": calldata });

    Ok(json)
}

/// Given a binary `Calldata` (wrapped within a JSON), decodes it into a JSON
pub fn decode_calldata(json: &Json) -> Result<Json, JsonError> {
    let data = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&data, "calldata")?;
    let mut calldata = CallData::new(&calldata);

    let mut abi = Vec::<Json>::new();
    let mut data = Vec::<Json>::new();

    while let Some(value) = calldata.next().into() {
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
        Primitive::Unit => unreachable!(),
    }
}

fn composite_as_json(c: &Composite) -> (Json, Json) {
    let slice: &[Value] = match c {
        Composite::Vec(inner) => inner.as_slice(),
    };

    if slice.is_empty() {
        return (Json::Null, Json::Array(std::vec::Vec::new()));
    }

    let mut types: Vec<Json> = Vec::new();
    let mut values: Vec<Json> = Vec::new();

    for elem in slice {
        let (ty, value) = value_as_json(elem);

        types.push(ty);
        values.push(value);
    }

    // TODO: assert that all `types` are the same
    let ty = types.pop().unwrap();

    (Json::Array(vec![ty]), Json::Array(values))
}

fn value_byte_size(ty: &Json, value: &Json) -> Result<usize, JsonError> {
    if ty.is_array() {
        let types = ty.as_array().unwrap();
        assert_eq!(types.len(), 1);

        let ty = &types[0];

        // we initialize `byte_size` for the `length` marker.
        let mut byte_size = 1;

        let json = json!({ "calldata": value });
        let elems = json::as_array(&json, "calldata")?;

        for elem in elems {
            byte_size += value_byte_size(ty, elem)?;
        }

        return Ok(byte_size);
    }

    let ty = as_str!(ty)?;

    let size = match ty {
        "bool" => bool::max_byte_size(),
        "i8" => i8::max_byte_size(),
        "u8" => u8::max_byte_size(),
        "i16" => i16::max_byte_size(),
        "u16" => u16::max_byte_size(),
        "i32" => i32::max_byte_size(),
        "u32" => u32::max_byte_size(),
        "i64" => i64::max_byte_size(),
        "u64" => u64::max_byte_size(),
        "amount" => Amount::max_byte_size(),
        "address" => Address::max_byte_size(),
        _ => {
            return Err(JsonError::InvalidField {
                field: "abi".to_string(),
                reason: format!("invalid ABI type: `{}`", ty),
            })
        }
    };

    Ok(size)
}

fn encode_value(ty: &Json, value: &Json) -> Result<Value, JsonError> {
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

fn encode_array(ty: &Json, value: &Json) -> Result<Value, JsonError> {
    debug_assert!(ty.is_array());

    let types = ty.as_array().unwrap();
    assert_eq!(types.len(), 1);

    let ty = &types[0];

    let json = json!({ "calldata": value });
    let elems = json::as_array(&json, "calldata")?;

    let mut vec = svm_sdk_std::Vec::with_capacity(10);

    for elem in elems {
        let elem = encode_value(ty, elem)?;

        vec.push(elem);
    }

    let c = Composite::Vec(vec);

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
