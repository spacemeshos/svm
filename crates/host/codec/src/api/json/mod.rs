//! JSON API

mod call;
mod error;
mod inputdata;
mod receipt;
pub(crate) mod serde_types;
mod spawn;

pub use call::{decode_call, encode_call, encode_call_raw};
pub use error::JsonError;
pub use inputdata::{decode_inputdata, encode_inputdata};
pub use receipt::decode_receipt;
pub use spawn::{decode_spawn, encode_spawn};

use serde::Deserialize;
use serde_json::{json, Value as Json};

use svm_types::{Gas, ReceiptLog};

fn gas_to_json(gas: &Gas) -> i64 {
    if gas.is_some() {
        gas.unwrap() as _
    } else {
        -1
    }
}

fn logs_to_json(logs: &[ReceiptLog]) -> Vec<Json> {
    logs.iter()
        .map(|log| {
            let data = unsafe { String::from_utf8_unchecked(log.as_bytes().to_vec()) };

            json!({
                "data": data,
            })
        })
        .collect()
}

fn get_field<T>(json: &mut Json, name: &str) -> Result<T, JsonError>
where
    T: for<'a> Deserialize<'a>,
{
    let object = json.as_object_mut().ok_or(JsonError::NotAnObject)?;
    let value = object
        .get_mut(name)
        .ok_or(JsonError::MissingField {
            field_name: name.to_string(),
        })?
        .take();
    serde_json::from_value(value).map_err(|_| JsonError::InvalidField {
        path: name.to_string(),
    })
}

fn parse_json(json_str: &str) -> Result<Json, JsonError> {
    let json_deserializer = &mut serde_json::Deserializer::from_str(json_str);
    let value = serde_path_to_error::deserialize(json_deserializer)?;
    Ok(value)
}
