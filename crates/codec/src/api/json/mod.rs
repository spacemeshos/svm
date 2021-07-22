//! JSON API

mod call;
mod calldata;
mod deploy;
mod error;
mod receipt;
mod spawn;
mod wrappers;

pub use call::{json_call_to_bytes, unwrap_binary_json_call};
pub use calldata::{decode_calldata, encode_calldata};
pub use deploy::deploy_template;
pub use error::JsonError;
pub use receipt::decode_receipt;
pub use spawn::{decode_spawn, encode_spawn};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as Json};

use svm_types::{Address, Gas, ReceiptLog, State};

pub(crate) trait JsonSerdeUtils: Serialize + for<'a> Deserialize<'a> {
    fn to_json(self) -> Json {
        serde_json::to_value(self).unwrap()
    }

    fn from_json_str(json_str: &str) -> Result<Self, JsonError> {
        let json_deserializer = &mut serde_json::Deserializer::from_str(json_str);
        let item = serde_path_to_error::deserialize(json_deserializer)?;
        Ok(item)
    }
}

pub(crate) fn to_bytes(json: &Json) -> Result<Vec<u8>, JsonError> {
    match serde_json::to_string(&json) {
        Ok(s) => Ok(s.into_bytes()),
        Err(e) => Err(JsonError::InvalidJson {
            line: e.line(),
            column: e.column(),
        }),
    }
}

pub(crate) fn as_array<'a>(json: &'a Json, field: &str) -> Result<&'a Vec<Json>, JsonError> {
    let v: &Json = &json[field];

    v.as_array().ok_or(JsonError::InvalidField {
        path: field.to_string(),
    })
}

pub(crate) fn bytes_to_str(bytes: &[u8]) -> String {
    hex::encode_upper(bytes)
}

pub(crate) fn addr_to_str(addr: &Address) -> String {
    bytes_to_str(addr.as_slice())
}

pub(crate) fn state_to_str(state: &State) -> String {
    bytes_to_str(state.as_slice())
}

pub(crate) fn gas_to_json(gas: &Gas) -> i64 {
    if gas.is_some() {
        gas.unwrap() as _
    } else {
        -1
    }
}

pub(crate) fn logs_to_json(logs: &[ReceiptLog]) -> Vec<Json> {
    logs.iter()
        .map(|log| {
            let msg = unsafe { String::from_utf8_unchecked(log.msg.clone()) };

            json!({
                "msg": msg,
                "code": log.code
            })
        })
        .collect()
}
