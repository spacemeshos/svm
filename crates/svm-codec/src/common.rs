use std::io::Cursor;

use crate::{Field, ParseError, ReadExt};

pub fn encode_string(s: &str, w: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    let length = bytes.len();

    assert!(length <= std::u8::MAX as usize);

    w.push(length as u8);
    w.extend_from_slice(&bytes);
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
