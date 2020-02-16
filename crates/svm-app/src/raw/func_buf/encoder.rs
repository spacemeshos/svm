use super::super::{encode_varuint14, Field, Nibble, NibbleWriter};

pub fn encode_func_buf(buf: &[u8], writer: &mut NibbleWriter) {
    let len = buf.len();

    assert!(len <= std::u16::MAX as usize);

    encode_varuint14(len as u16, writer);

    writer.write_bytes(buf)
}

#[cfg(test)]
mod tests {
    use crate::nib;

    use super::*;

    #[test]
    fn encode_func_buf_empty_buf() {
        let buf = vec![];
        let mut writer = NibbleWriter::new();

        encode_func_buf(&buf[..], &mut writer);

        assert!(!writer.is_byte_aligned());

        let padding = nib!(0b_0000_1111);
        writer.write(&[padding]);

        let bytes = writer.bytes();
        assert_eq!(vec![0b_0000_1111], bytes);
    }

    #[test]
    fn encode_func_buf_non_empty_buf() {
        let buf = vec![0x10, 0x20, 0x30, 0x40];
        let mut writer = NibbleWriter::new();

        encode_func_buf(&buf[..], &mut writer);

        // `varuint14` encoding of `buf.len()`:
        let buf_len_encoding = 0b_01_00_0100;

        let bytes = writer.bytes();
        assert_eq!(vec![buf_len_encoding, 0x10, 0x20, 0x30, 0x40], bytes);
    }
}
