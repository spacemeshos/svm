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

use serde_json::{json, Value};

use svm_types::{Address, Gas, ReceiptLog, State};

pub(crate) trait TypeInformation {
    fn type_of_field_as_str(field: &str) -> Option<&str>;
}

//fn parse_json(json: &str) -> Result<Value, JsonError> {
//    serde_json::from_str(json).map_err(|_| JsonError::InvalidJson)
//}

/// A blob of binary data that is encoded with Base16.
#[derive(Clone, Debug)]
pub struct HexBlob(Vec<u8>);

impl serde::Serialize for HexBlob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(hex::encode_upper(&self.0).as_str())
    }
}

impl<'de> serde::Deserialize<'de> for HexBlob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let s: String = serde::Deserialize::deserialize(deserializer)?;
        if s.len() % 2 != 0 {
            return Err(D::Error::custom("Bad length"));
        }
        hex::decode(s)
            .map(|bytes| Self(bytes))
            .map_err(|_| D::Error::custom("Bad hex"))
    }
}

pub(crate) fn to_bytes(json: &Value) -> Result<Vec<u8>, JsonError> {
    match serde_json::to_string(&json) {
        Ok(s) => Ok(s.into_bytes()),
        Err(e) => Err(JsonError::Unknown(format!("{}", e))),
    }
}

pub(crate) fn as_string(json: &Value, field: &str) -> Result<String, JsonError> {
    let v: &Value = &json[field];

    v.as_str()
        .map(|v| v.to_string())
        .ok_or(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a(n) string", v),
        })
}

pub(crate) fn as_array<'a>(json: &'a Value, field: &str) -> Result<&'a Vec<Value>, JsonError> {
    let v: &Value = &json[field];

    v.as_array().ok_or(JsonError::InvalidField {
        field: field.to_string(),
        reason: format!("value `{}` isn't a(n)n Array", v),
    })
}

pub(crate) fn bytes_to_str(bytes: &[u8]) -> String {
    hex::encode_upper(bytes)
}

pub(crate) fn str_to_bytes(v: &str, field: &str) -> Result<Vec<u8>, JsonError> {
    if v.len() % 2 == 1 {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should be of even length".to_string(),
        });
    }

    if v.chars().any(|c| c.is_ascii_hexdigit() == false) {
        return Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: "value should have only hex digits".to_string(),
        });
    }

    fn chars_as_byte(c1: char, c2: char) -> u8 {
        let c1 = c1.to_digit(16).unwrap() as u8;
        let c2 = c2.to_digit(16).unwrap() as u8;

        (c1 << 4) | c2
    }

    let bytes = v
        .chars()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|slice| {
            let (c1, c2) = (slice[0], slice[1]);
            chars_as_byte(c1, c2)
        })
        .collect();

    Ok(bytes)
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

pub(crate) fn logs_to_json(logs: &[ReceiptLog]) -> Vec<Value> {
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

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

    fn as_blob(json: &Value, field: &str) -> Result<Vec<u8>, JsonError> {
        let v = as_string(json, field)?;
        str_to_bytes(&v, field)
    }

    #[test]
    fn json_as_blob_valid() {
        let json = json!({
            "blob": "1DB30F"
        });

        let blob = as_blob(&json, "blob").unwrap();
        assert_eq!(blob, vec![0x1D, 0xB3, 0x0F])
    }

    #[test]
    fn json_as_blob_invalid_chars() {
        let json = json!({
            "blob": "NOT HEX!"
        });

        let err = as_blob(&json, "blob").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "blob".to_string(),
                reason: "value should have only hex digits".to_string(),
            }
        );
    }

    #[test]
    fn json_as_blob_invalid_odd_length() {
        let json = json!({
            "blob": "A0B"
        });

        let err = as_blob(&json, "blob").unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "blob".to_string(),
                reason: "value should be of even length".to_string(),
            }
        );
    }
}
