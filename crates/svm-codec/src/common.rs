use std::io::Cursor;

use svm_types::Address;

use crate::{Field, ParseError, ReadExt};

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
    let mut bytes = n.to_be_bytes();

    w.extend_from_slice(&bytes);
}

pub fn encode_u32_be(n: u32, w: &mut Vec<u8>) {
    let mut bytes = n.to_be_bytes();

    w.extend_from_slice(&bytes);
}

/// Decoders

#[must_use]
pub fn decode_address(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<Address, ParseError> {
    cursor
        .read_bytes(Address::len())
        .map(|bytes| Address::from(&bytes[..]))
        .map_err(|_| ParseError::NotEnoughBytes(field))
}

#[must_use]
pub fn decode_string(
    cursor: &mut Cursor<&[u8]>,
    len_field: Field,
    field: Field,
) -> Result<String, ParseError> {
    match cursor.read_byte() {
        Err(..) => Err(ParseError::EmptyField(field)),
        Ok(byte) => {
            let length = byte as usize;

            match cursor.read_bytes(length) {
                Err(..) => Err(ParseError::NotEnoughBytes(field)),
                Ok(vec) => {
                    String::from_utf8(vec).map_err(|_e| ParseError::InvalidUTF8String(field))
                }
            }
        }
    }
}

pub fn decode_u16_be(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u16, ParseError> {
    cursor
        .read_u16_be()
        .map_err(|_| ParseError::NotEnoughBytes(field))
}

pub fn decode_u32_be(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u32, ParseError> {
    cursor
        .read_u32_be()
        .map_err(|_| ParseError::NotEnoughBytes(field))
}
