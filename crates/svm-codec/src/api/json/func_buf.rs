use crate::{
    api::json::{self, JsonError},
    nibble::NibbleWriter,
};

use svm_abi_encoder::Encoder;
use svm_sdk::value::{
    Address, AddressOwned, Array, Composite, Primitive, PubKey256, PubKey256Owned, Value,
};
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

    if abi.len() != data.len() {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "`abi` and `data` must be of the same length".to_string(),
        });
    }

    let mut buf = Vec::new();

    for (i, (ty, raw)) in abi.iter().zip(data).enumerate() {
        encode_value(ty, raw, i, &mut buf)?;
    }

    Ok(buf)
}

fn encode_value(
    ty: &json::Value,
    raw: &json::Value,
    pos: usize,
    buf: &mut Vec<u8>,
) -> Result<(), JsonError> {
    match ty {
        json::Value::String(..) => encode_primitive(ty, raw, pos, buf),
        json::Value::Array(..) => encode_array(ty, raw, pos, buf),
        _ => Err(JsonError::InvalidField {
            field: "abi".to_string(),
            reason: "`abi` expects only `string` or `Array` items".to_string(),
        }),
    }
}

macro_rules! str_as_primitive {
    ($raw:expr, $ty:ty, $field:expr) => {{
        let bytes = json::str_to_bytes($raw, $field)?;

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

macro_rules! do_encode_primitive {
    ("address", $raw:expr, $field:expr, $buf:expr) => {{
        let prim = str_as_primitive!(&$raw, AddressOwned, $field)?;
        prim.encode($buf);

        Ok(())
    }};

    ("pubkey256", $raw:expr, $field:expr, $buf:expr) => {{
        let pkey = str_as_primitive!($raw, PubKey256Owned, $field)?;
        pkey.encode($buf);

        Ok(())
    }};

    ($ty:expr, $field:expr, $buf:expr) => {{
        return Err(JsonError::InvalidField {
            field: "abi".to_string(),
            reason: format!("invalid ABI type `{}`", $ty),
        });
    }};
}

macro_rules! do_encode_array {
    ($raw_array:expr, $ty:ty, $pos:expr, $buf:expr) => {{
        let mut vec = Vec::new();

        for (i, raw) in $raw_array.iter().enumerate() {
            let field = format!("data[{}], item={}", $pos, i);
            let raw = as_str(raw, &field)?;

            let prim = str_as_primitive!(&raw, $ty, &field)?;
            vec.push(prim);
        }

        vec.encode($buf);

        Ok(())
    }};
}

fn encode_array(
    ty: &json::Value,
    raw: &json::Value,
    pos: usize,
    buf: &mut Vec<u8>,
) -> Result<(), JsonError> {
    let ty = as_array(ty, &format!("abi[{}]", pos))?;

    if ty.len() != 1 {
        return Err(JsonError::InvalidField {
            field: "abi".to_string(),
            reason: "`Array` items must be of length = 1`".to_string(),
        });
    }

    let ty = as_str(&ty[0], &format!("abi[{}][0]", pos))?;
    let raw = as_array(raw, &format!("data[{}][0]", pos))?;

    match ty {
        "address" => do_encode_array!(raw, AddressOwned, pos, buf),
        "pubkey256" => do_encode_array!(raw, PubKey256Owned, pos, buf),
        _ => Err(JsonError::InvalidField {
            field: format!("data[{}]", pos),
            reason: format!("unsupported `Array` of type: {}", ty),
        }),
    }
}

fn encode_primitive(
    ty: &json::Value,
    raw: &json::Value,
    pos: usize,
    buf: &mut Vec<u8>,
) -> Result<(), JsonError> {
    let ty = as_str(ty, &format!("abi[{}]", pos))?;
    let raw = as_str(raw, &format!("data[{}]", pos))?;
    let field = format!("data[{}]", pos);

    match ty {
        "address" => do_encode_primitive!("address", raw, &field, buf),
        "pubkey256" => do_encode_primitive!("pubkey256", raw, &field, buf),
        _ => do_encode_primitive!(ty, &field, buf),
    }
}

pub fn decode_func_buf(_raw: &json::Value) -> Result<Vec<u8>, JsonError> {
    todo!()
}

fn as_str<'a>(raw: &'a json::Value, field: &str) -> Result<&'a str, JsonError> {
    match raw.as_str() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't a string", raw),
        }),
        Some(raw) => Ok(raw),
    }
}

fn as_array<'a>(raw: &'a json::Value, field: &str) -> Result<&'a Vec<json::Value>, JsonError> {
    match raw.as_array() {
        None => Err(JsonError::InvalidField {
            field: field.to_string(),
            reason: format!("value `{}` isn't an Array", raw),
        }),
        Some(raw) => Ok(raw),
    }
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

            let vec = Box::leak(Box::new(vec));

            unsafe { core::mem::transmute::<*const u8, &[u8; $n]>(vec.as_ptr()) }
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
            "data": [[addr!("1020304050"), addr!("60708090A0")]]
        });

        let bytes = encode_func_buf(&json).unwrap();
        let decoder = Decoder::new();
        let mut cursor = Cursor::new(&bytes);
        let actual = decoder.decode_value(&mut cursor).unwrap();

        let addr1 = Value::Primitive(Primitive::Address(Address(extend!(
            vec![0x10, 0x20, 0x30, 0x40, 0x50],
            20
        ))));

        let addr2 = Value::Primitive(Primitive::Address(Address(extend!(
            vec![0x60, 0x70, 0x80, 0x90, 0xA0],
            20
        ))));

        let addrs = [addr1, addr2];
        let expected = Value::Composite(Composite::Array(&addrs));
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn json_encode_func_buf_pubkey256_array() {
        let json = json!({
            "abi": [["pubkey256"]],
            "data": [[pkey!("10203040"), pkey!("A0B0C0D0")]]
        });

        let bytes = encode_func_buf(&json).unwrap();
        let decoder = Decoder::new();
        let mut cursor = Cursor::new(&bytes);
        let actual = decoder.decode_value(&mut cursor).unwrap();

        let pkey1 = Value::Primitive(Primitive::PubKey256(PubKey256(extend!(
            vec![0x10, 0x20, 0x30, 0x40],
            32
        ))));

        let pkey2 = Value::Primitive(Primitive::PubKey256(PubKey256(extend!(
            vec![0xA0, 0xB0, 0xC0, 0xD0],
            32
        ))));

        let pkeys = [pkey1, pkey2];
        let expected = Value::Composite(Composite::Array(&pkeys));
        assert_eq!(expected, actual);
    }
}
