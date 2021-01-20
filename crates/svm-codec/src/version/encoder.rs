use svm_nibble::NibbleWriter;

/// Encodes version
pub fn encode_version(mut version: u32, w: &mut NibbleWriter) {
    let mut byte = msb(version);

    while has_more(byte) {
        w.write_byte(byte);

        version = shift(version);
        byte = msb(version);
    }

    w.write_byte(byte);
}

fn has_more(byte: u8) -> bool {
    byte & 0b_1000_0000 != 0
}

fn msb(n: u32) -> u8 {
    (((n & 0xFF_00_00_00) >> 24) & 0xFF) as u8
}

fn shift(n: u32) -> u32 {
    n << 8
}

#[cfg(test)]
mod tests {
    use super::*;

    /*     fn assert_encoding(version: u32, expected: Vec<u8>) {
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
    } */
}
