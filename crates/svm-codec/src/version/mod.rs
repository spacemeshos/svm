mod decoder;
mod encoder;

pub use decoder::decode_version;
pub use encoder::encode_version;

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    use crate::ParseError;

    fn encode(version: u32) -> Vec<u8> {
        let mut buf = Vec::new();

        encode_version(version, &mut buf);

        buf
    }

    fn decode(bytes: Vec<u8>) -> u32 {
        let mut cursor = Cursor::new(&bytes[..]);

        decode_version(&mut cursor).unwrap()
    }

    #[test]
    fn encode_decode_version() {
        /*         let bytes = encode(0);
        let decoded = decode(bytes);
        assert_eq!(decoded, 0);

        let bytes = encode(0b_10100000_00000011_00000000_00000000);
        let decoded = decode(bytes);
        assert_eq!(decoded, 0b_10100000_00000011);

        let bytes = encode(0b_10100000_11000111_01000100_00000000);
        let decoded = decode(bytes);
        assert_eq!(decoded, 0b_10100000_11000111_01000100); */

        let bytes = encode(0b_10100000_11000111_10000100_0000001);
        dbg!(bytes);
        /*         let decoded = decode(bytes);
        assert_eq!(decoded, 0b_10100000_11000111_10000100_0000001); */
    }
}
