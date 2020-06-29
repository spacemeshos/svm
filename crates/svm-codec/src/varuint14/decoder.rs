use crate::{Field, NibbleIter, ParseError};

use bit_vec::BitVec;

/// Decodes a `varuint14` field into a `u16`.
pub fn decode_varuint14(iter: &mut NibbleIter, field: Field) -> Result<u16, ParseError> {
    let preamble = iter.next();

    if preamble.is_none() {
        return Err(ParseError::NotEnoughBytes(field));
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

    let nibble_count = nibble_count - 1;

    for _ in 0..nibble_count {
        match iter.next() {
            None => return Err(ParseError::NotEnoughBytes(field)),
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
    }
}
