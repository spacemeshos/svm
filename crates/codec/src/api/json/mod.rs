//! JSON API

mod call;
mod calldata;
mod deploy;
mod error;
mod receipt;
mod spawn;

pub(crate) mod wrappers;

pub use call::{decode_call, encode_call, encode_call_raw};
pub use calldata::{decode_calldata, encode_calldata};
pub use deploy::deploy_template;
pub use error::JsonError;
pub use receipt::decode_receipt;
pub use spawn::{decode_spawn, encode_spawn};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as Json};

use svm_types::{Gas, ReceiptLog};

/// Provides very simple utilily functions to working with [`serde_json::Value`]
/// in an easy way.
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

/// Converts a [`Json`] value to a UTF-8 valid [`Vec<u8>`] JSON representation.
///
/// # Panics
///
/// Panics if serialization type implementations fail or `json` contains a map
/// with non-string keys.
pub(crate) fn to_bytes(json: &Json) -> Vec<u8> {
    serde_json::to_string(&json)
        .expect("JSON serialization error")
        .into_bytes()
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
