use super::{Field, Nibble, NibbleIter};
use crate::error::ParseError;

use bit_vec::BitVec;

pub fn parse_func_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    let preamble = iter.next();

    if preamble.is_none() {
        return Err(ParseError::NotEnoughBytes(Field::FuncIndex));
    }

    let [hint_0, hint_1, msb_0, msb_1] = preamble.unwrap().bits();

    let (nibble_count, padding) = {
        match (hint_0, hint_1) {
            (false, false) => (1, 14),
            (false, true) => (2, 10),
            (true, false) => (3, 6),
            (true, true) => (4, 2),
        }
    };

    let mut bits = BitVec::from_elem(padding, false);

    // the 1st nibble always contains 2 bits of `func index`
    bits.push(msb_0);
    bits.push(msb_1);

    for _ in 0..nibble_count - 1 {
        match iter.next() {
            None => return Err(ParseError::NotEnoughBytes(Field::FuncIndex)),
            Some(nibble) => {
                for bit in nibble.bits().iter() {
                    bits.push(*bit);
                }
            }
        }
    }

    assert_eq!(16, bits.len());

    let bytes = bits.to_bytes();

    let mut be_bytes: [u8; 2] = [0; 2];
    be_bytes[0] = bytes[0];
    be_bytes[1] = bytes[1];

    let func_idx = u16::from_be_bytes(be_bytes);
    Ok(func_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_func_index_empty() {
        let vec = vec![];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::NotEnoughBytes(Field::FuncIndex));

        assert_eq!(expected, parse_func_index(&mut iter));
    }

    #[test]
    fn parse_func_index_one_nibble() {
        let vec = vec![0b00_11_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = parse_func_index(&mut iter).unwrap();
        assert_eq!(0b11, func_idx);
    }

    #[test]
    fn parse_func_index_2_nibbles() {
        let vec = vec![0b01_11_0111];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = parse_func_index(&mut iter).unwrap();
        assert_eq!(0b11_0111, func_idx);
    }

    #[test]
    fn parse_func_index_3_nibbles() {
        let vec = vec![0b10_11_0111, 0b0101_0000];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = parse_func_index(&mut iter).unwrap();
        assert_eq!(0b11_0111_0101, func_idx);
    }

    #[test]
    fn parse_func_index_4_nibbles() {
        let vec = vec![0b11_11_0111, 0b0101_0011];
        let mut iter = NibbleIter::new(&vec[..]);

        let func_idx = parse_func_index(&mut iter).unwrap();
        assert_eq!(0b11_0111_0101_0011, func_idx);
    }

    #[test]
    fn parse_func_index_not_enough_bytes() {
        let vec = vec![0b10_11_0111];
        let mut iter = NibbleIter::new(&vec[..]);

        let expected = Err(ParseError::NotEnoughBytes(Field::FuncIndex));

        assert_eq!(expected, parse_func_index(&mut iter));
    }
}
