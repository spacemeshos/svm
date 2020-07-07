use serde_json::Value;

use crate::{
    api::json::{self, JsonError},
    nibble::NibbleWriter,
};

use svm_sdk::value::Array;

///
/// ```json
/// {
///   abi: [['Address'], ['PubKey256'], 'Address'],
///   data: [
///     [ ['0x1020..'], ... ],
///     [ ['0x3040..'], ... ],
///     '0x4050'
///   ]
/// }
/// ```
pub fn encode_func_buf(json: &Value) -> Result<Vec<u8>, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    Ok(Vec::new())
}

pub fn decode_func_buf(json: &Value) -> Result<Vec<u8>, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_missing_abi() {
        todo!()
    }

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_missing_data() {
        todo!()
    }

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_address_primitive() {
        todo!()
    }

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_pubkey256_primitive() {
        todo!()
    }

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_address_array() {
        todo!()
    }

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_pubkey256_array() {
        todo!()
    }
}
