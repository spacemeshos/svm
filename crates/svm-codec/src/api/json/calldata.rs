use crate::api::json::{self, JsonError};
use crate::api::raw;
use crate::nibble::{NibbleIter, NibbleWriter};

use svm_types::WasmValue;

use serde_json::{json, Value};

pub fn encode_calldata(json: &json::Value) -> Result<Value, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    if abi.len() != data.len() {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "`abi` and `data` must be of the same length".to_string(),
        });
    }

    let mut args = Vec::new();
    let mut buf_abi = Vec::new();
    let mut buf_data = Vec::new();

    for (ty, raw) in abi.iter().zip(data) {
        match ty.as_str() {
            Some("i32") => args.push(format!("{}i32", raw)),
            Some("i64") | Some("amount") => args.push(format!("{}i64", raw)),
            _ => {
                buf_abi.push(ty.clone());
                buf_data.push(raw.clone())
            }
        }
    }

    let func_args = json::encode_func_args(&args)?;
    let func_buf = encode_func_buf(buf_abi, buf_data)?;

    let json = json!({
        "func_args": json::bytes_to_str(&func_args),
        "func_buf": json::bytes_to_str(&func_buf),
    });

    Ok(json)
}

fn encode_func_buf(abi: Vec<Value>, data: Vec<Value>) -> Result<Vec<u8>, JsonError> {
    let abi = Value::Array(abi);
    let data = Value::Array(data);

    json::encode_func_buf(&json!({
        "abi": abi,
        "data": data
    }))
}

pub fn decode_calldata(json: &json::Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "func_args")?;
    let data = json::str_to_bytes(&data, "func_args")?;

    let mut iter = NibbleIter::new(&data);
    let func_args: Vec<_> = raw::decode_func_args(&mut iter)
        .unwrap()
        .iter()
        .map(|v| match v {
            WasmValue::I32(v) => format!("{}i32", v),
            WasmValue::I64(v) => format!("{}i64", v),
        })
        .collect();

    let data = json::as_string(json, "func_buf")?;
    let func_buf = json::decode_func_buf(&json!({ "data": data }))?;

    let json = json!({
        "func_args": func_args,
        "func_buf": func_buf,
    });

    Ok(json)
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
                "func_buf": [
                    {"address": "102030405060708090a0112233445566778899aa"},
                    {"pubkey256": "1020304050607080102030405060708010203040506070801020304050607080"}
                ],
            })
        );
    }
}
