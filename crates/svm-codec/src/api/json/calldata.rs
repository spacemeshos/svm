use serde_json::json;

use svm_abi_encoder::Encoder;
use svm_sdk::value::{Address, AddressOwned, Composite, Primitive, Value};

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

pub fn encode_calldata(json: &json::Value) -> Result<Vec<u8>, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    if abi.len() != data.len() {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "`abi` and `data` must be of the same length".to_string(),
        });
    }

    let mut buf = Vec::new();

    for (ty, raw) in abi.iter().zip(data) {
        let ty = as_str!(ty)?;

        let value = encode_value(ty, raw)?;
        value.encode(&mut buf);
    }

    Ok(buf)
}

pub fn decode_calldata(json: &json::Value) -> Result<json::Value, JsonError> {
    let data = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&data, "calldata")?;

    let json = json!({ "calldata": calldata });

    Ok(json)
}

fn encode_value<'a>(ty: &'a str, value: &json::Value) -> Result<Value<'static>, JsonError> {
    if ty.starts_with("[") {
        return encode_array(ty, value);
    }

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
            let bytes: &[u8] = addr.as_slice();

            let addr: Address = bytes.into();
            let addr: AddressOwned = addr.to_owned();
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

fn encode_array(ty: &str, value: &json::Value) -> Result<Value<'static>, JsonError> {
    debug_assert!(ty.starts_with("["));

    if !ty.ends_with("]") {
        return Err(JsonError::InvalidField {
            field: "calldata".to_string(),
            reason: format!(
                "ABI type that starts with `[` should end with a `]` (got: {})",
                ty
            ),
        });
    }

    let ty: &str = &ty[1..ty.len() - 1];
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

    #[test]
    pub fn encode_calldata_sanity() {
        let addr = "102030405060708090A0112233445566778899AA";
        let pkey = "1020304050607080102030405060708010203040506070801020304050607080";

        let json = json!({
            "abi": ["i32", "amount", "address", "i64", "pubkey256"],
            "data": [10, 20, addr, 30, pkey]
        });

        let calldata = encode_calldata(&json).unwrap();
        let decoded = json::decode_calldata(&calldata).unwrap();

        assert_eq!(
            decoded,
            json!({
                "func_args": ["10i32", "20i64", "30i64"],
                "calldata": [
                    {"address": "102030405060708090a0112233445566778899aa"},
                    {"pubkey256": "1020304050607080102030405060708010203040506070801020304050607080"}
                ],
            })
        );
    }
}
