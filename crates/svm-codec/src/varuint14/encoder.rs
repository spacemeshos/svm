/// Encodes `u16` into `varuint14`. The value of `num` should fit within 14 bits.
pub fn encode_varuint14(num: u16, w: &mut Vec<u8>) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    /*     #[test]
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
    } */
}
