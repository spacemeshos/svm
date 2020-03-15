mod decoder;
mod encoder;

pub use decoder::decode_varuint14;
pub use encoder::encode_varuint14;

#[cfg(test)]
mod tests {
    use crate::nib;

    use super::super::{helpers, Field, NibbleIter, NibbleWriter};
    use super::{decode_varuint14, encode_varuint14};

    fn assert_encode_decode(num: u16) {
        let mut w = NibbleWriter::new();

        encode_varuint14(num, &mut w);

        let data = w.into_bytes();
        let mut iter = NibbleIter::new(&data[..]);

        // choosing an arbitrary `varuint14` field.
        let field = Field::NameLength;

        let decoded = decode_varuint14(&mut iter, field).unwrap();
        assert_eq!(num, decoded);

        iter.ensure_eof();
    }

    #[test]
    fn encode_decode_varuint14() {
        assert_encode_decode(0);
        assert_encode_decode(0xFF);
        assert_encode_decode((1 << 14) - 1);
    }
}
