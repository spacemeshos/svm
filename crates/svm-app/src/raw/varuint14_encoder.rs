use crate::nib;

use super::{Nibble, NibbleWriter};

pub fn encode_varuint14(num: u16, writer: &mut NibbleWriter) {
    match num {
        0..=0b_0000_0011 => {
            // `num` cosumes between 0 to 2 bits inclusive.
            // encoding hint: `00`

            let nib = nib!(num as u8);
            writer.write(&[nib]);
        }
        0b_0000_0100..=0b_0011_1111 => {
            // `num` consumes between 3 to 6 bits inclusive.
            // encoding hint: `01`

            let nib1 = nib!((0b_01_00 | (num >> 4)) as u8);
            let nib2 = nib!((num & 0x0F) as u8);

            writer.write(&[nib1, nib2]);
        }
        0b_0100_0000..=0b_0011_1111_1111 => {
            // `num` consumes between 7 to 10 bits inclusive.
            // encoding hint: `10`
            let nib1 = nib!((0b_10_00 | (num >> 8)) as u8);
            let nib2 = nib!(((num >> 4) & 0x0F) as u8);
            let nib3 = nib!(((num >> 0) & 0x0F) as u8);

            writer.write(&[nib1, nib2, nib3]);
        }
        0b_0000_0100_0000_0000..=0b_0011_1111_1111_1111 => {
            // `num` consumes between 11 to 14 bits inclusive.
            // encoding hint: `11`

            let nib1 = nib!((0b_11_00 | (num >> 12)) as u8);
            let nib2 = nib!(((num >> 8) & 0x0F) as u8);
            let nib3 = nib!(((num >> 4) & 0x0F) as u8);
            let nib4 = nib!(((num >> 0) & 0x0F) as u8);

            writer.write(&[nib1, nib2, nib3, nib4]);
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
        let mut writer = NibbleWriter::new();

        encode_varuint14(num, &mut writer);

        // before calling `writer.bytes()` we need `writer`
        // to to have an even number of nibbles.
        writer.write(&[padding]);

        let bytes = writer.bytes();
        assert_eq!(vec![0b_00_10_1111], bytes);
    }

    #[test]
    fn encode_varuint14_6_bits() {
        let num = 0b_0010_1110;
        let mut writer = NibbleWriter::new();

        encode_varuint14(num, &mut writer);

        let bytes = writer.bytes();
        assert_eq!(vec![0b_01_10_1110], bytes);
    }

    #[test]
    fn encode_varuint14_10_bits() {
        let num = 0b_0011_1001_1110;
        let mut writer = NibbleWriter::new();

        encode_varuint14(num, &mut writer);

        // before calling `writer.bytes()` we need `writer`
        // to to have an even number of nibbles.
        let padding = nib!(0b_0000_1111);
        writer.write(&[padding]);

        let bytes = writer.bytes();
        assert_eq!(vec![0b_10_11_1001, 0b_1110_1111], bytes);
    }

    #[test]
    fn encode_varuint14_14_bits() {
        let num = 0b_0011_0010_1001_1110;
        let mut writer = NibbleWriter::new();

        encode_varuint14(num, &mut writer);

        let bytes = writer.bytes();
        assert_eq!(vec![0b_1111_0010, 0b_1001_1110], bytes);
    }

    #[test]
    #[should_panic(expected = "`num` 16384 is out-of-range (exceeds 14 bits)")]
    fn encode_varuint14_15_bits_panic() {
        let num = 0b_0100_0000_0000_0000;
        let mut writer = NibbleWriter::new();

        encode_varuint14(num, &mut writer);

        let _ = writer.bytes();
    }
}
