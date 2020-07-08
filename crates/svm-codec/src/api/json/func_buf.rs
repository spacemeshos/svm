use crate::{
    api::json::{self, JsonError},
    nibble::NibbleWriter,
};

use svm_abi_encoder::Encoder;
use svm_sdk::value::{AddressOwned, Array, Primitive, PubKey256Owned, Value};
use svm_sdk::{self as sdk};

///
/// ```json
/// {
///   abi: [['address'], ['pubkey256'], 'address'],
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

    // todo: return `JsonError`
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
    match ty {
        json::Value::String(..) => encode_primitive(ty, value, pos, buf),
        json::Value::Array(..) => encode_array(ty, value, pos, buf),
        _ => todo!("invalid input"),
    }
}

macro_rules! encode_primitive_array {
    ($values:expr, $ty:ty, $func:expr, $pos:expr, $buf:expr) => {{
        let mut vec = Vec::new();

        for (i, value) in $values.iter().enumerate() {
            let field = format!("data[{}], item={}", $pos, i);
            let value = value.as_str().unwrap();

            let prim = $func(value, &field)?;
            vec.push(prim);

            vec.encode($buf);
        }
    }};
}

fn encode_array(
    ty: &json::Value,
    value: &json::Value,
    pos: usize,
    buf: &mut Vec<u8>,
) -> Result<(), JsonError> {
    let ty: &Vec<json::Value> = ty.as_array().unwrap();

    // todo: return `JsonError`
    assert_eq!(ty.len(), 1);

    let ty = ty[0].as_str().unwrap();
    let values = value.as_array().unwrap();

    match ty {
        "address" => encode_primitive_array!(values, AddressOwned, str_as_addr, pos, buf),
        "pubkey256" => encode_primitive_array!(values, PubKey256Owned, str_as_pubkey256, pos, buf),
        _ => todo!(),
    }

    Ok(())
}

fn encode_primitive(
    ty: &json::Value,
    value: &json::Value,
    pos: usize,
    buf: &mut Vec<u8>,
) -> Result<(), JsonError> {
    let ty = ty.as_str().unwrap();
    let value = value.as_str().unwrap();
    let field = format!("data[{}]", pos);

    match ty {
        "address" => encode_addr(value, &field, buf)?,
        "pubkey256" => encode_pubkey256(value, &field, buf)?,
        _ => {
            return Err(JsonError::InvalidField {
                field: "abi".to_string(),
                reason: format!("invalid ABI type {}", ty),
            })
        }
    }

    Ok(())
}

macro_rules! str_as_primitive {
    ($s:expr, $ty:ty, $field:expr) => {{
        let bytes = json::str_to_bytes($s, $field)?;

        if bytes.len() != <$ty>::size() {
            return Err(JsonError::InvalidField {
                field: $field.to_string(),
                reason: format!("value should be exactly {} hex digits", <$ty>::size() * 2),
            });
        }

        let ty: $ty = bytes.into();
        Ok(ty)
    }};
}

fn str_as_addr(s: &str, field: &str) -> Result<AddressOwned, JsonError> {
    str_as_primitive!(s, AddressOwned, field)
}

fn str_as_pubkey256(s: &str, field: &str) -> Result<PubKey256Owned, JsonError> {
    str_as_primitive!(s, PubKey256Owned, field)
}

fn encode_addr(value: &str, field: &str, buf: &mut Vec<u8>) -> Result<(), JsonError> {
    let addr: AddressOwned = str_as_addr(value, field)?;
    addr.encode(buf);

    Ok(())
}

fn encode_pubkey256(value: &str, field: &str, buf: &mut Vec<u8>) -> Result<(), JsonError> {
    let bytes = json::str_to_bytes(value, &field)?;
    let pkey: svm_sdk::value::PubKey256 = (&bytes[..]).into();

    pkey.encode(buf);

    Ok(())
}

pub fn decode_func_buf(json: &json::Value) -> Result<Vec<u8>, JsonError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use svm_abi_decoder::{Cursor, Decoder};

    macro_rules! extend {
        ($bytes:expr, $n:expr) => {{
            assert_eq!($n % $bytes.len(), 0);

            let m = $n / $bytes.len();
            let vec = (&$bytes[..]).repeat(m);

            unsafe { core::mem::transmute::<*const u8, &[u8; $n]>(&vec[0]) }
        }};
    }

    macro_rules! repeat_str {
        ($str:expr, $size:expr) => {{
            let len = $str.len();
            assert_eq!(len % 2, 0);

            let nbytes = len / 2;
            assert_eq!($size % nbytes, 0);

            let n = $size / nbytes;
            $str.repeat(n)
        }};
    }

    macro_rules! addr {
        ($str:expr) => {{
            repeat_str!($str, 20)
        }};
    }

    macro_rules! pkey {
        ($str:expr) => {{
            repeat_str!($str, 32)
        }};
    }

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
            "abi": ["address"],
            "data": [addr!("1020304050")]
        });

        let bytes = encode_func_buf(&json).unwrap();
        let decoder = Decoder::new();
        let mut cursor = Cursor::new(&bytes);
        let actual = decoder.decode_value(&mut cursor).unwrap();

        let expected = Value::Primitive(Primitive::Address(Address(extend!(
            vec![0x10, 0x20, 0x30, 0x40, 0x50],
            20
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn json_encode_func_buf_pubkey256_primitive() {
        let json = json!({
            "abi": ["pubkey256"],
            "data": [pkey!("1020304050607080")]
        });

        let bytes = encode_func_buf(&json).unwrap();
        let decoder = Decoder::new();
        let mut cursor = Cursor::new(&bytes);
        let actual = decoder.decode_value(&mut cursor).unwrap();

        let expected = Value::Primitive(Primitive::PubKey256(PubKey256(extend!(
            vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
            32
        ))));

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn json_encode_func_buf_address_array() {
        let json = json!({
            "abi": [["address"]],
            "data": [[addr!("1020304050"), addr!("60708090A)")]]
        });

        let bytes = encode_func_buf(&json);
        dbg!(bytes);
        // let decoder = Decoder::new();
        // let mut cursor = Cursor::new(&bytes);
        // let actual = decoder.decode_value(&mut cursor).unwrap();

        // let expected = Value::Primitive(Primitive::Address(Address(extend!(
        //     vec![0x10, 0x20, 0x30, 0x40, 0x50],
        //     20
        // ))));

        // assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    pub fn json_encode_func_buf_pubkey256_array() {
        todo!()
    }
}
