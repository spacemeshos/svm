mod decoder;
mod encoder;

pub use decoder::decode_version;
pub use encoder::encode_version;

#[cfg(test)]
mod tests {
    use crate::api::raw::{decode_version, encode_version};
    use crate::nibble::{NibbleIter, NibbleWriter};

    fn assert_encode_decode(version: u32) {
        let mut w = NibbleWriter::new();

        encode_version(version, &mut w);

        let data = w.into_bytes();
        let mut iter = NibbleIter::new(&data[..]);

        let decoded = decode_version(&mut iter).unwrap();
        assert_eq!(version, decoded);

        assert!(iter.ensure_eof().is_ok());
    }

    #[test]
    fn encode_decode_version() {
        assert_encode_decode(0);
        assert_encode_decode(std::u8::MAX.into());
        assert_encode_decode(std::u16::MAX.into());
        assert_encode_decode(1 << 20);
    }
}
