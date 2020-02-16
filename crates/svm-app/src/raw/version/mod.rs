mod decoder;
mod encoder;

pub use decoder::decode_version;
pub use encoder::encode_version;

#[cfg(test)]
mod tests {
    use crate::nib;

    use super::super::{NibbleIter, NibbleWriter};
    use super::*;

    fn assert_encode_decode(version: u32) {
        let mut writer = NibbleWriter::new();

        encode_version(version, &mut writer);

        if writer.is_byte_aligned() == false {
            let padding = nib!(0);
            writer.write(&[padding]);
        }

        let data = writer.bytes();
        let mut iter = NibbleIter::new(&data[..]);

        let decoded = decode_version(&mut iter).unwrap();
        assert_eq!(version, decoded);
    }

    #[test]
    fn encode_decode_version() {
        assert_encode_decode(0);
        assert_encode_decode(std::u8::MAX.into());
        assert_encode_decode(std::u16::MAX.into());
        assert_encode_decode(1 << 20);
    }
}
