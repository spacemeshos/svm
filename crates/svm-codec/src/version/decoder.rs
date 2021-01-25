use std::io::{Cursor, Read};

use crate::api::raw::Field;
use crate::error::ParseError;

/// Decodes the `version` into an `u32`
pub fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let mut version = 0;

    let mut byte = read_byte(cursor)?;

    while has_more(byte) {
        version = append(version, byte)?;

        byte = read_byte(cursor)?;
    }

    version = append(version, byte)?;

    Ok(version)
}

fn read_byte(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParseError> {
    let mut buf = [0; 1];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::Version));
    }

    Ok(buf[0])
}

fn has_more(byte: u8) -> bool {
    byte & 0b_1000_0000 != 0
}

fn append(n: u32, byte: u8) -> Result<u32, ParseError> {
    if let Some(n) = n.checked_shl(8) {
        if let Some(n) = n.checked_add(byte.into()) {
            return Ok(n);
        }
    }

    Err(ParseError::TooManyBytes(Field::Version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_version_empty_input() {
        let bytes = Vec::new();
        let mut cursor = Cursor::new(&bytes[..]);

        let expected = ParseError::EmptyField(Field::Version);
        let actual = decode_version(&mut cursor).unwrap_err();

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_version_two_bytes() {
        let bytes = vec![0b_11010000, 0b_00000011];
        let mut cursor = Cursor::new(&bytes[..]);

        let expected = 0b_11010000_00000011;
        let actual = decode_version(&mut cursor).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_version_three_bytes() {
        let bytes = vec![0b_11010000, 0b_11000011, 0b_00000011];
        let mut cursor = Cursor::new(&bytes[..]);

        let expected = 0b_11010000_11000011_00000011;
        let actual = decode_version(&mut cursor).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_version_four_bytes() {
        let bytes = vec![0b_11010000, 0b_11000011, 0b_10000011, 0b_00000101];
        let mut cursor = Cursor::new(&bytes[..]);

        let expected = 0b_11010000_11000011_10000011_00000101;
        let actual = decode_version(&mut cursor).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_version_too_many_bytes() {
        let bytes = vec![
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b0000_0000,
        ];
        let mut cursor = Cursor::new(&bytes[..]);

        let expected = ParseError::TooManyBytes(Field::Version);
        let actual = decode_version(&mut cursor).unwrap_err();

        assert_eq!(expected, actual);
    }
}
