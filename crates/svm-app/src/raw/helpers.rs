use super::{Field, NibbleIter, NibbleWriter};
use crate::{error::ParseError, nib, raw, types::WasmValue};

use svm_common::Address;

pub fn bytes(writer: &mut NibbleWriter) -> Vec<u8> {
    // before calling `writer.bytes()` we must make sure
    // that its number of nibbles is even. If it's not, we pad it with one extra nibble.

    if writer.is_byte_aligned() == false {
        let padding = nib!(0);
        writer.write(&[padding]);
    }

    writer.bytes()
}

/// Making sure there are no nibbles left to read,
/// except for an optional padding nibble, used to even the number of nibbles.
pub fn ensure_eof(iter: &mut NibbleIter) -> Result<(), ParseError> {
    if iter.is_byte_aligned() == false {
        let nib = iter.next();
        debug_assert!(nib.is_some());
    };

    match iter.next() {
        None => Ok(()),
        Some(..) => Err(ParseError::ExpectedEOF),
    }
}

/// Encoders

pub fn encode_func_buf(buf: &[u8], writer: &mut NibbleWriter) {
    raw::encode_func_buf(buf, writer);
}

pub fn encode_func_args(args: &[WasmValue], writer: &mut NibbleWriter) {
    raw::encode_func_args(args, writer);
}

pub fn encode_version(version: u32, writer: &mut NibbleWriter) {
    raw::encode_version(version, writer);
}

pub fn encode_varuint14(num: u16, writer: &mut NibbleWriter) {
    raw::encode_varuint14(num, writer);
}

pub fn encode_address(addr: &Address, writer: &mut NibbleWriter) {
    let bytes = addr.bytes();
    writer.write_bytes(&bytes[..]);
}

pub fn encode_string(s: &str, writer: &mut NibbleWriter) {
    let bytes = s.as_bytes();
    let length = bytes.len();

    assert!(length <= std::u16::MAX as usize);

    encode_varuint14(length as u16, writer);

    writer.write_bytes(&bytes[..]);
}

/// Decoders

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
