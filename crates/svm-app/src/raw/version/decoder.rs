use super::super::{Field, NibbleIter};
use crate::error::ParseError;

use bit_vec::BitVec;

/// Decodes the version into `u32` bytes.
pub fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    let mut bits = BitVec::new();

    for nibble in iter {
        let [_msb_0, msb_1, msb_2, msb_3] = nibble.bits();

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

    if bits.len() > 32 {
        return Err(ParseError::TooManyBytes(Field::Version));
    }

    let padding = 32 - bits.len();

    if padding > 0 {
        let mut new_bits = BitVec::from_elem(padding, false);

        new_bits.append(&mut bits);
        bits = new_bits;
    };

    let bytes = bits.to_bytes();
    assert_eq!(4, bytes.len());

    let mut be_bytes: [u8; 4] = [0; 4];

    for (i, byte) in bytes.iter().enumerate() {
        be_bytes[i] = *byte;
    }

    let ver = u32::from_be_bytes(be_bytes);
    Ok(ver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_version_no_nibbles() {
        let vec = vec![];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::EmptyField(Field::Version));

        assert_eq!(expected, decode_version(&mut iter));
    }

    #[test]
    fn decode_version_one_nibble() {
        let vec = vec![0b0101_1111];
        let mut iter = NibbleIter::new(&vec[..]);

        let ver = decode_version(&mut iter).unwrap();
        assert_eq!(0b101, ver);
    }

    #[test]
    fn decode_version_two_nibbles() {
        let vec = vec![0b1101_0011];
        let mut iter = NibbleIter::new(&vec[..]);

        let ver = decode_version(&mut iter).unwrap();
        assert_eq!(0b101_011, ver);
    }

    #[test]
    fn decode_version_three_nibbles() {
        let vec = vec![0b1101_1011, 0b0010_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        let ver = decode_version(&mut iter).unwrap();
        assert_eq!(0b101_011_010, ver);
    }

    #[test]
    fn decode_version_too_many_bytes() {
        let vec = vec![
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b1000_1000,
            0b0000_0000,
        ];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::TooManyBytes(Field::Version));

        assert_eq!(expected, decode_version(&mut iter));
    }
}
