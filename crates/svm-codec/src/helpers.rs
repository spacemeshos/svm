use std::io::{Cursor, Read};

use byteorder::{BigEndian, ByteOrder};

use svm_types::Address;

use crate::api::raw::{self, Field};
use crate::error::ParseError;

/// Encoders

pub fn encode_address(addr: &Address, w: &mut Vec<u8>) {
    let bytes = addr.bytes();

    w.extend_from_slice(&bytes);
}

pub fn encode_string(s: &str, w: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    let length = bytes.len();

    assert!(length <= std::u8::MAX as usize);

    w.push(length as u8);
    w.extend_from_slice(&bytes);
}

pub fn encode_u16_be(n: u16, w: &mut Vec<u8>) {
    let mut buf = vec![0; 2];

    BigEndian::write_u16(&mut buf, n);

    w.extend_from_slice(&buf);
}

pub fn encode_u32_be(n: u32, w: &mut Vec<u8>) {
    let mut buf = vec![0; 4];

    BigEndian::write_u32(&mut buf, n);

    w.extend_from_slice(&buf);
}

/// Decoders

#[must_use]
pub fn decode_address(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<Address, ParseError> {
    let mut buf = [0; Address::len()];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(field));
    }

    let addr = buf.into();

    Ok(addr)
}

#[must_use]
pub fn decode_string(
    cursor: &mut Cursor<&[u8]>,
    len_field: Field,
    field: Field,
) -> Result<String, ParseError> {
    let mut buf = [0; 1];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::EmptyField(field));
    }

    let length = buf[0] as usize;
    let mut buf = Vec::with_capacity(length);

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(field));
    }

    String::from_utf8(buf).or_else(|_e| Err(ParseError::InvalidUTF8String(field)))
}

pub fn decode_u16_be(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u16, ParseError> {
    let mut buf = [0; 2];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(field));
    }

    let n = BigEndian::read_u16(&buf);

    Ok(n)
}

pub fn decode_u32_be(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u32, ParseError> {
    let mut buf = [0; 4];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(field));
    }

    let n = BigEndian::read_u32(&buf);

    Ok(n)
}
