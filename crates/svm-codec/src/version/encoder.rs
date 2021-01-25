/// Encodes version
pub fn encode_version(mut version: u32, w: &mut Vec<u8>) {
    let mut byte = msb(version);

    while has_more(byte) {
        w.push(byte);

        version = shift(version);
        byte = msb(version);
    }

    w.push(byte);
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

    fn assert_encoding(version: u32, expected: Vec<u8>) {
        let mut actual = Vec::new();

        encode_version(version, &mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn encode_version_0() {
        let version = 0;
        let expected = vec![0];

        assert_encoding(version, expected);
    }

    #[test]
    fn encode_version_two_bytes() {
        let version = 0b_10010100_00000011_00000000_00000000;
        let expected = vec![0b_10010100, 0b_0011];

        assert_encoding(version, expected);
    }
}
