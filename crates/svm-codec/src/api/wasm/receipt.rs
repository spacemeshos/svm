use serde_json::Value;

use super::wasm_buf_apply;
use crate::{
    api,
    api::json::{self, JsonError},
};

pub fn decode_receipt(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, |json: &Value| {
        let json = api::json::decode_receipt(json)?;

        api::json::to_bytes(&json)
    })
}
