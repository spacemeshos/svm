use crate::nib;

use super::super::{Nibble, NibbleWriter};

pub fn encode_version(mut version: u32, w: &mut NibbleWriter) {
    let mut has_more = true;
    let mut more_bit = 0;

    let mut nibbles = Vec::new();

    while has_more {
        let (next_ver, lsb_2, lsb_1, lsb_0) = next_triple_bits(version);

        let nib = build_nibble(more_bit, lsb_2, lsb_1, lsb_0);
        nibbles.push(nib);

        version = next_ver;
        more_bit = 1;

        has_more = version > 0;
    }

    // since we've scanned `version` from `lsb` to `msb` order,
    // we need to reverse `nibbles` prior calling `w` with them.
    let nibbles: Vec<Nibble> = nibbles.drain(..).rev().collect();

    w.write(&nibbles[..]);
}

fn next_triple_bits(version: u32) -> (u32, u8, u8, u8) {
    let lsb_0 = ((version & 0b_000_0001) >> 0) as u8;
    let lsb_1 = ((version & 0b_000_0010) >> 1) as u8;
    let lsb_2 = ((version & 0b_000_0100) >> 2) as u8;
    let new_ver = version >> 3;

    (new_ver, lsb_2, lsb_1, lsb_0)
}

fn build_nibble(more_bit: u8, lsb_2: u8, lsb_1: u8, lsb_0: u8) -> Nibble {
    let byte = (more_bit << 3) | (lsb_2 << 2) | (lsb_1 << 1) | (lsb_0);

    nib!(byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_encoding(version: u32, expected: Vec<u8>) {
        let mut w = NibbleWriter::new();

        encode_version(version, &mut w);

        assert_eq!(expected, w.into_bytes());
    }

    fn assert_encoding_with_padding(version: u32, padding: Nibble, expected: Vec<u8>) {
        let mut w = NibbleWriter::new();

        encode_version(version, &mut w);

        // we pad a nibble so thaht `w` will
        // hold an even number of nibbles, otherwise `w.bytes()` will fail.
        w.write(&[padding]);

        assert_eq!(expected, w.into_bytes());
    }

    #[test]
    fn encode_version_0() {
        let version = 0;
        let padding = nib!(0b_1111);
        let expected = vec![0b_0000_1111];

        assert_encoding_with_padding(version, padding, expected);
    }

    #[test]
    fn encode_version_1() {
        let version = 0b_0001;
        let padding = nib!(0b_1111);
        let expected = vec![0b_0001_1111];

        assert_encoding_with_padding(version, padding, expected);
    }

    #[test]
    fn encode_version_2() {
        let version = 0b_0010;
        let padding = nib!(0b_1111);
        let expected = vec![0b_0010_1111];

        assert_encoding_with_padding(version, padding, expected);
    }

    #[test]
    fn encode_version_3() {
        let version = 0b_0011;
        let padding = nib!(0b_1111);
        let expected = vec![0b_0011_1111];

        assert_encoding_with_padding(version, padding, expected);
    }

    #[test]
    fn encode_version_8() {
        let version = 0b_1000;
        let expected = vec![0b_1001_0000];

        assert_encoding(version, expected);
    }

    #[test]
    fn encode_version_010_001() {
        let version = 0b_00_010_001;
        let expected = vec![0b_1010_0001];

        assert_encoding(version, expected);
    }

    #[test]
    fn encode_version_100_001_010() {
        let version = 0b_100_001_010;
        let padding = nib!(0b_1111);
        let expected = vec![0b_1100_1001, 0b_0010_1111];

        assert_encoding_with_padding(version, padding, expected);
    }
}
