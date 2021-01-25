use std::io::{Cursor, Read};

use crate::{api::raw::Field, error::ParseError};

use bit_vec::BitVec;

/// Decodes a `varuint14` field into a `u16`.
pub fn decode_varuint14(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u16, ParseError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*     #[test]
    fn decode_varuint14_empty() {
        let vec = vec![];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::NotEnoughBytes(Field::FuncIndex));

        assert_eq!(expected, decode_varuint14(&mut iter, Field::FuncIndex));
    }

    #[test]
    fn decode_varuint14_one_nibble() {
        let vec = vec![0b00_11_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = decode_varuint14(&mut iter, Field::FuncIndex).unwrap();
        assert_eq!(0b11, func_idx);
    }

    #[test]
    fn decode_varuint14_2_nibbles() {
        let vec = vec![0b01_11_0111];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = decode_varuint14(&mut iter, Field::FuncIndex).unwrap();
        assert_eq!(0b11_0111, func_idx);
    }

    #[test]
    fn decode_varuint14_3_nibbles() {
        let vec = vec![0b10_11_0111, 0b0101_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = decode_varuint14(&mut iter, Field::FuncIndex).unwrap();
        assert_eq!(0b11_0111_0101, func_idx);
    }

    #[test]
    fn decode_varuint14_4_nibbles() {
        let vec = vec![0b11_11_0111, 0b0101_0011];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = decode_varuint14(&mut iter, Field::FuncIndex).unwrap();
        assert_eq!(0b11_0111_0101_0011, func_idx);
    }

    #[test]
    fn decode_varuint14_not_enough_bytes() {
        let vec = vec![0b10_11_0111];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::NotEnoughBytes(Field::FuncIndex));

        assert_eq!(expected, decode_varuint14(&mut iter, Field::FuncIndex));
    } */
}
