use super::{Field, NibbleIter, NibbleWriter};
use crate::{error::ParseError, raw, types::WasmValue};

use byteorder::{BigEndian, ByteOrder};

use svm_common::Address;

/// Encoders

pub fn encode_wasm_value(wasm_value: &WasmValue, w: &mut NibbleWriter) {
    raw::encode_wasm_value(wasm_value, w);
}

pub fn encode_func_buf(buf: &[u8], w: &mut NibbleWriter) {
    raw::encode_func_buf(buf, w);
}

pub fn encode_func_args(args: &[WasmValue], w: &mut NibbleWriter) {
    raw::encode_func_args(args, w);
}

pub fn encode_version(version: u32, w: &mut NibbleWriter) {
    raw::encode_version(version, w);
}

pub fn encode_varuint14(num: u16, w: &mut NibbleWriter) {
    raw::encode_varuint14(num, w);
}

pub fn encode_address(addr: &Address, w: &mut NibbleWriter) {
    let bytes = addr.bytes();
    w.write_bytes(&bytes[..]);
}

pub fn encode_string(s: &str, w: &mut NibbleWriter) {
    let bytes = s.as_bytes();
    let length = bytes.len();

    assert!(length <= std::u16::MAX as usize);

    encode_varuint14(length as u16, w);

    w.write_bytes(&bytes[..]);
}

pub fn encode_u32_be(n: u32, w: &mut NibbleWriter) {
    let mut buf = vec![0; 4];
    BigEndian::write_u32(&mut buf, n);

    w.write_bytes(&buf[..]);
}

/// Decoders

pub fn decode_wasm_value(iter: &mut NibbleIter) -> Result<WasmValue, ParseError> {
    todo!()
}

#[must_use]
pub fn decode_func_buf(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    raw::decode_func_buf(iter)
}

#[must_use]
pub fn decode_func_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    raw::decode_func_args(iter)
}

#[must_use]
pub fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    raw::decode_version(iter)
}

#[must_use]
pub fn decode_varuint14(iter: &mut NibbleIter, field: Field) -> Result<u16, ParseError> {
    raw::decode_varuint14(iter, field)
}

#[must_use]
pub fn decode_address(iter: &mut NibbleIter, field: Field) -> Result<Address, ParseError> {
    let bytes = iter.read_bytes(Address::len());

    if bytes.len() != Address::len() {
        return Err(ParseError::NotEnoughBytes(field));
    }

    let addr = Address::from(&bytes[..]);
    Ok(addr)
}

#[must_use]
pub fn decode_string(
    iter: &mut NibbleIter,
    len_field: Field,
    field: Field,
) -> Result<String, ParseError> {
    let length = decode_varuint14(iter, len_field)? as usize;

    if length == 0 {
        return Err(ParseError::EmptyField(len_field));
    }

    let bytes = iter.read_bytes(length);

    if bytes.len() != length {
        return Err(ParseError::NotEnoughBytes(field));
    }

    String::from_utf8(bytes).or_else(|_e| Err(ParseError::InvalidUTF8String(field)))
}

pub fn decode_u32_be(iter: &mut NibbleIter, field: Field) -> Result<u32, ParseError> {
    let bytes = iter.read_bytes(4);

    if bytes.len() != 4 {
        return Err(ParseError::NotEnoughBytes(field));
    }

    let n = BigEndian::read_u32(&bytes[..]);

    Ok(n)
}
