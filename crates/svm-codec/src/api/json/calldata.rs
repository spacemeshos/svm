use serde_json::{json, Value};
use svm_abi_encoder::Encoder;

use crate::api::json::{self, JsonError};
use crate::api::raw;

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
        let ty = ty.as_str().unwrap();
        let raw = raw.as_str().unwrap();

        let _value = encode_value(ty, raw, &mut buf)?;
    }

    Ok(buf)
}

pub fn decode_calldata(json: &json::Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "calldata")?;
    let calldata = json::str_to_bytes(&data, "calldata")?;

    let json = json!({ "calldata": calldata });

    Ok(json)
}

fn encode_value(ty: &str, value: &str, buf: &mut Vec<u8>) -> Result<(), JsonError> {
    let json = json!({ "calldata": value });

    macro_rules! encode {
        ($func:ident) => {{
            json::$func(&json, "calldata")?.encode(buf)
        }};
    }

    match ty {
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

            let addr: svm_sdk::value::Address = bytes.into();
            addr.encode(buf)
        }
        "array" => todo!(),
        _ => {
            return Err(JsonError::InvalidField {
                field: "data".to_string(),
                reason: "`abi` and `data` must be of the same length".to_string(),
            })
        }
    };

    Ok(())
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
