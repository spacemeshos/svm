mod decoder;
mod encoder;

pub use decoder::decode_func_buf;
pub use encoder::encode_func_buf;

#[cfg(test)]
mod tests {
    use crate::nib;

    use super::super::{helpers, NibbleIter, NibbleWriter};
    use super::{decode_func_buf, encode_func_buf};

    fn assert_encode_decode(buf: Vec<u8>) {
        let mut writer = NibbleWriter::new();

        encode_func_buf(&buf[..], &mut writer);

        let data = helpers::bytes(&mut writer);
        let mut iter = NibbleIter::new(&data);

        let decoded = decode_func_buf(&mut iter).unwrap();
        assert_eq!(buf, decoded);

        helpers::ensure_eof(&mut iter);
    }

    #[test]
    fn encode_decode_func_buf() {
        assert_encode_decode(vec![]);
        assert_encode_decode(vec![0x10, 0x20, 0x30]);
    }
}
