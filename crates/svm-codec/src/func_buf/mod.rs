mod decoder;
mod encoder;

pub use decoder::decode_func_buf;
pub use encoder::encode_func_buf;

#[cfg(test)]
mod tests {
    use crate::nibble::{NibbleIter, NibbleWriter};
    use crate::{decode_func_buf, encode_func_buf};

    fn assert_encode_decode(buf: Vec<u8>) {
        let mut w = NibbleWriter::new();

        encode_func_buf(&buf[..], &mut w);

        let data = w.into_bytes();
        let mut iter = NibbleIter::new(&data[..]);

        let decoded = decode_func_buf(&mut iter).unwrap();
        assert_eq!(buf, decoded);

        assert!(iter.ensure_eof().is_ok());
    }

    #[test]
    fn encode_decode_func_buf() {
        assert_encode_decode(vec![]);
        assert_encode_decode(vec![0x10, 0x20, 0x30]);
    }
}
