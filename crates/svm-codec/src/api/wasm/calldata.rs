use serde_json::Value;

use super::wasm_buf_apply;
use crate::{api, api::json::JsonError};

pub fn encode_calldata(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, |json: &Value| {
        let json = api::json::encode_calldata(json)?;

        api::json::to_bytes(&json)
    })
}

pub fn decode_calldata(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, |json: &Value| {
        let json = api::json::decode_calldata(json)?;

        api::json::to_bytes(&json)
    })
}
