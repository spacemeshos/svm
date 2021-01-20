use svm_nibble::NibbleIter;

use crate::api::raw::Field;
use crate::error::ParseError;

/// Decodes the `version` into an `u32`
pub fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    let mut version = 0;

    let mut byte = iter.read_byte();

    while has_more(byte) {
        version = append(version, byte);

        byte = iter.read_byte();
    }

    version = append(version, byte);

    Ok(version)
}

fn has_more(byte: u8) -> bool {
    byte & 0b_1000_0000 != 0
}

fn append(mut n: u32, byte: u8) -> u32 {
    n <<= 8;
    n += (byte as u32);

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_version_empty_input() {
        let vec = vec![];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::EmptyField(Field::Version));

        assert_eq!(expected, decode_version(&mut iter));
    }

    #[test]
    fn decode_version_one_byte() {
        let vec = vec![0b0101_1111];
        let mut iter = NibbleIter::new(&vec[..]);

        let ver = decode_version(&mut iter).unwrap();
        assert_eq!(0b101, ver);
    }

    #[test]
    fn decode_version_two_bytes() {
        let vec = vec![0b1101_0011];
        let mut iter = NibbleIter::new(&vec[..]);

        let ver = decode_version(&mut iter).unwrap();
        assert_eq!(0b101_011, ver);
    }

    #[test]
    fn decode_version_three_bytes() {
        let vec = vec![0b1101_1011, 0b0010_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        let ver = decode_version(&mut iter).unwrap();
        assert_eq!(0b101_011_010, ver);
    }

    #[test]
    fn decode_version_four_bytes() {
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
