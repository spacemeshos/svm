use crate::nib;

use super::{Nibble, NibbleWriter};

pub fn encode_varuint16(num: u16, w: &mut NibbleWriter) {
    match num {
        0..=0x00_0F => {
            // 1 nibble
            let a = nibble_pos(num, 0);
            w.write(&[a]);
        }
        0x00_10..=0xFF => {
            // 2 nibbles
            let a = nibble_pos(num, 1);
            let b = nibble_pos(num, 0);

            w.write(&[a, b]);
        }
        0x01_00..=0x0F_FF => {
            // 3 nibbles
            let a = nibble_pos(num, 2);
            let b = nibble_pos(num, 1);
            let c = nibble_pos(num, 0);

            w.write(&[a, b, c]);
        }
        _ => {
            // 4 nibbles
            let a = nibble_pos(num, 3);
            let b = nibble_pos(num, 2);
            let c = nibble_pos(num, 1);
            let d = nibble_pos(num, 0);

            w.write(&[a, b, c, d]);
        }
    }
}

#[inline]
fn nibble_pos(num: u16, pos: usize) -> Nibble {
    match pos {
        0 => nib!((num & 0x0F) as u8),
        1 => nib!((num & 0xF0) as u8),
        2 => nib!(((num & 0x0F_00) >> 8) as u8),
        3 => nib!(((num & 0xF0_00) >> 8) as u8),
        _ => unreachable!(),
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn encode_varuint16_1_nibble() {
        let num = 0b_0000_0011;
        let mut write = NibbleWriter::new();

        encode_varuint16(num, &mut buf);
    }
}
