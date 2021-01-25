mod decoder;
mod encoder;

pub use decoder::decode_varuint14;
pub use encoder::encode_varuint14;

#[cfg(test)]
mod tests {
    /*     #[test]
    fn encode_decode_varuint14() {
        assert_encode_decode(0);
        assert_encode_decode(0xFF);
        assert_encode_decode((1 << 14) - 1);
    } */
}
