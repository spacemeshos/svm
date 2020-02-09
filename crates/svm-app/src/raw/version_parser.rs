use std::io::{Cursor, Read};

use super::{Field, Nibble, NibbleIter};
use crate::error::ParseError;

use bit_vec::BitVec;

pub fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let mut iter = NibbleIter::new(cursor);
    let mut bits = BitVec::new();

    for mut nibble in iter {
        let [msb_0, msb_1, msb_2, msb_3] = nibble.bits();

        bits.push(msb_1);
        bits.push(msb_2);
        bits.push(msb_3);

        if nibble.is_msb_off() {
            break;
        }
    }

    if bits.len() == 0 {
        return Err(ParseError::EmptyField(Field::Version));
    }

    let n = bits.len() % 8;

    if n > 0 {
        let padding = 8 - n;
        let mut new_bits = BitVec::from_elem(padding, false);

        new_bits.append(&mut bits);
        bits = new_bits;
    };

    let bytes = bits.to_bytes();

    if bytes.len() > 4 {
        return Err(ParseError::TooManyBytes(Field::Version));
    }

    let mut be_bytes: [u8; 4] = [0; 4];

    let off = 4 - bytes.len();

    for (i, byte) in bytes.iter().enumerate() {
        be_bytes[off + i] = *byte;
    }

    let ver = u32::from_be_bytes(be_bytes);

    Ok(ver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_no_nibbles() {
        let vec = vec![];
        let mut cursor = Cursor::new(&vec[..]);

        let expected = Err(ParseError::EmptyField(Field::Version));

        assert_eq!(expected, parse_version(&mut cursor));
    }

    #[test]
    fn parse_version_one_nibble() {
        let vec = vec![0b0101_1111];
        let mut cursor = Cursor::new(&vec[..]);

        let ver = parse_version(&mut cursor).unwrap();
        assert_eq!(0b101, ver);
    }

    #[test]
    fn parse_version_two_nibbles() {
        let vec = vec![0b1101_0011];
        let mut cursor = Cursor::new(&vec[..]);

        let ver = parse_version(&mut cursor).unwrap();
        assert_eq!(0b101_011, ver);
    }

    #[test]
    fn parse_version_three_nibbles() {
        let vec = vec![0b1101_1011, 0b0010_0000];
        let mut cursor = Cursor::new(&vec[..]);

        let ver = parse_version(&mut cursor).unwrap();
        assert_eq!(0b101_011_010, ver);
    }

    #[test]
    fn parse_version_too_many_bytes() {
        let vec = vec![
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b0000_0000,
        ];
        let mut cursor = Cursor::new(&vec[..]);

        let expected = Err(ParseError::TooManyBytes(Field::Version));

        assert_eq!(expected, parse_version(&mut cursor));
    }
}
