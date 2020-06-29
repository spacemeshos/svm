use crate::nib;
use crate::nibble::NibbleWriter;

/// Encodes `u16` into `varuint14`. The value of `num` should fit within 14 bits.
pub fn encode_varuint14(num: u16, w: &mut NibbleWriter) {
    match num {
        0..=0b_0000_0011 => {
            // `num` cosumes between 0 to 2 bits inclusive.
            // encoding hint: `00`

            let nib = nib!(num as u8);
            w.write(&[nib]);
        }
        0b_0000_0100..=0b_0011_1111 => {
            // `num` consumes between 3 to 6 bits inclusive.
            // encoding hint: `01`

            let nib1 = nib!((0b_01_00 | (num >> 4)) as u8);
            let nib2 = nib!((num & 0x0F) as u8);

            w.write(&[nib1, nib2]);
        }
        0b_0100_0000..=0b_0011_1111_1111 => {
            // `num` consumes between 7 to 10 bits inclusive.
            // encoding hint: `10`
            let nib1 = nib!((0b_10_00 | (num >> 8)) as u8);
            let nib2 = nib!(((num >> 4) & 0x0F) as u8);
            let nib3 = nib!(((num >> 0) & 0x0F) as u8);

            w.write(&[nib1, nib2, nib3]);
        }
        0b_0000_0100_0000_0000..=0b_0011_1111_1111_1111 => {
            // `num` consumes between 11 to 14 bits inclusive.
            // encoding hint: `11`

            let nib1 = nib!((0b_11_00 | (num >> 12)) as u8);
            let nib2 = nib!(((num >> 8) & 0x0F) as u8);
            let nib3 = nib!(((num >> 4) & 0x0F) as u8);
            let nib4 = nib!(((num >> 0) & 0x0F) as u8);

            w.write(&[nib1, nib2, nib3, nib4]);
        }
        _ => panic!(format!("`num` {} is out-of-range (exceeds 14 bits)", num)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_varuint14_2_bits() {
        let padding = nib!(0b_0000_1111);
        let num = 0b_0000_0010;
        let mut w = NibbleWriter::new();

        encode_varuint14(num, &mut w);

        // before calling `w.bytes()` we need `w`
        // to to have an even number of nibbles.
        w.write(&[padding]);

        assert_eq!(vec![0b_00_10_1111], w.into_bytes());
    }

    #[test]
    fn encode_varuint14_6_bits() {
        let num = 0b_0010_1110;
        let mut w = NibbleWriter::new();

        encode_varuint14(num, &mut w);

        assert_eq!(vec![0b_01_10_1110], w.into_bytes());
    }

    #[test]
    fn encode_varuint14_10_bits() {
        let num = 0b_0011_1001_1110;
        let mut w = NibbleWriter::new();

        encode_varuint14(num, &mut w);

        // before calling `w.bytes()` we need `w`
        // to to have an even number of nibbles.
        let padding = nib!(0b_0000_1111);
        w.write(&[padding]);

        assert_eq!(vec![0b_10_11_1001, 0b_1110_1111], w.into_bytes());
    }

    #[test]
    fn encode_varuint14_14_bits() {
        let num = 0b_0011_0010_1001_1110;
        let mut w = NibbleWriter::new();

        encode_varuint14(num, &mut w);

        assert_eq!(vec![0b_1111_0010, 0b_1001_1110], w.into_bytes());
    }

    #[test]
    #[should_panic(expected = "`num` 16384 is out-of-range (exceeds 14 bits)")]
    fn encode_varuint14_15_bits_panic() {
        let num = 0b_0100_0000_0000_0000;
        let mut w = NibbleWriter::new();

        encode_varuint14(num, &mut w);

        let _ = w.into_bytes();
    }
}
