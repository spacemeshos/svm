use std::io::{Cursor, Read};

use super::{Field, Nibble, NibbleIter};
use crate::error::ParseError;

use bit_vec::BitVec;

pub fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let mut iter = NibbleIter::new(cursor);
    let mut bits = BitVec::new();

    for mut nibble in iter.next() {
        let [_msb_0, msb_1, msb_2, msb_3] = nibble.bits();

        bits.push(msb_1);
        bits.push(msb_2);
        bits.push(msb_3);
    }

    todo!()
}
