use crate::{
    api::json::{self, JsonError},
    nibble::NibbleWriter,
};

use svm_abi_encoder::Encoder;
use svm_sdk::value::{Address, Array, Primitive, Value};
use svm_sdk::{self as sdk};

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
pub fn encode_func_buf(json: &json::Value) -> Result<Vec<u8>, JsonError> {
    let abi = json::as_array(json, "abi")?;
    let data = json::as_array(json, "data")?;

    assert_eq!(abi.len(), data.len());

    let mut buf = Vec::new();

    for (i, ty) in abi.iter().enumerate() {
        let value = &data[i];

        encode_value(ty, value, i, &mut buf)?;
    }

    Ok(buf)
}

fn encode_value(
    ty: &json::Value,
    value: &json::Value,
    pos: usize,
    buf: &mut Vec<u8>,
) -> Result<(), JsonError> {
    let field = format!("data[{}]", pos);
    let ty = ty.as_str().unwrap();
    let value = value.as_str().unwrap();

    let addr: svm_types::Address = json::str_as_addr(value, &field)?;
    let bytes = addr.as_slice();

    let addr: svm_sdk::value::Address = bytes.into();
    addr.encode(buf);

    Ok(())
}

pub fn decode_func_buf(json: &json::Value) -> Result<Vec<u8>, JsonError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    pub fn json_encode_func_buf_missing_abi() {
        let json = json!({});

        let err = encode_func_buf(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "abi".to_string(),
                reason: "value `null` isn\'t an Array".to_string(),
            }
        );
    }

    #[test]
    pub fn json_encode_func_buf_missing_data() {
        let json = json!({
            "abi": []
        });

        let err = encode_func_buf(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "data".to_string(),
                reason: "value `null` isn\'t an Array".to_string(),
            }
        );
    }

    #[test]
    pub fn json_encode_func_buf_address_primitive() {
        let json = json!({
            "abi": ["Address"],
            "data": ["10203040506070809000A0B0C0D0E0F0ABCDEFFF"]
        });

        let bytes = encode_func_buf(&json).unwrap();
        let decoder = svm_abi_decoder::Decoder::new();
        let mut cursor = svm_abi_decoder::Cursor::new(&bytes);
        let actual = decoder.decode_value(&mut cursor).unwrap();

        let expected = Value::Primitive(Primitive::Address(Address(&[
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0x00, 0xA0, 0xB0, 0xC0, 0xD0,
            0xE0, 0xF0, 0xAB, 0xCD, 0xEF, 0xFF,
        ])));

        assert_eq!(expected, actual);
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
