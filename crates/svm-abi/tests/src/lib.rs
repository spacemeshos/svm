//! This crate tests the encoding & decoding of a function buffer.
//! using SVM default ABI.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

#[cfg(test)]
mod tests {
    use svm_abi_decoder::{Cursor, Decoder};
    use svm_abi_encoder::Encoder;

    use svm_sdk::value::{Composite, Primitive, Value};
    use svm_sdk::Address;

    macro_rules! test {
        ($ty:ty, $rust_value:expr) => {{
            let rust_value: $ty = $rust_value.clone();
            let abi_value: Value = rust_value.into();

            let mut buf_native = Vec::new();
            let mut buf_abi_value = Vec::new();

            let rust_value: $ty = $rust_value.clone();
            rust_value.encode(&mut buf_native);
            abi_value.encode(&mut buf_abi_value);

            // Asserting that encoding a Rust rust_value number
            // gives the same results when encoding the corresponding `Value` wrapper.
            assert_eq!(buf_native, buf_abi_value);

            let mut cursor = Cursor::new(&buf_native);
            let decoder = Decoder::new();
            let abi_value = decoder.decode_value(&mut cursor).unwrap();

            let n: $ty = abi_value.into();
            assert_eq!(n, $rust_value);
        }};
    }

    #[test]
    fn encode_decode_bool() {
        test!(bool, true);
        test!(bool, false);
    }

    #[test]
    fn encode_decode_i8() {
        test!(i8, -5);
        test!(i8, 5);

        test!(i8, 0);
        test!(i8, -1);
        test!(i8, std::i8::MIN as i8);
        test!(i8, std::i8::MAX as i8);
    }

    #[test]
    fn encode_decode_u8() {
        test!(u8, 5);

        test!(u8, 0);
        test!(u8, std::u8::MIN as u8);
        test!(u8, std::u8::MAX as u8);
    }

    #[test]
    fn encode_decode_i16() {
        test!(i16, -5);
        test!(i16, 5);

        test!(i16, 0);
        test!(i16, -1);

        test!(i16, std::i8::MIN as i16);
        test!(i16, std::i8::MAX as i16);

        test!(i16, std::u8::MIN as i16);
        test!(i16, std::u8::MAX as i16);

        test!(i16, std::i16::MIN as i16);
        test!(i16, std::i16::MAX as i16);
    }

    #[test]
    fn encode_decode_u16() {
        test!(u16, 5);
        test!(u16, 0);

        test!(u16, 127);

        test!(u16, std::i8::MIN as u16);
        test!(u16, std::i8::MAX as u16);

        test!(u16, std::u8::MIN as u16);
        test!(u16, std::u8::MAX as u16);

        test!(u16, std::i16::MAX as u16);
        test!(u16, std::u16::MAX as u16);
    }

    #[test]
    fn encode_decode_i32() {
        test!(i32, 5);
        test!(i32, 0);
        test!(i32, -1);

        test!(i32, std::i8::MIN as i32);
        test!(i32, std::i8::MAX as i32);

        test!(i32, std::u8::MIN as i32);
        test!(i32, std::u8::MAX as i32);

        test!(i32, std::i16::MIN as i32);
        test!(i32, std::i16::MAX as i32);

        test!(i32, std::u16::MIN as i32);
        test!(i32, std::u16::MAX as i32);

        test!(i32, std::i32::MIN as i32);
        test!(i32, std::i32::MAX as i32);
    }

    #[test]
    fn encode_decode_u32() {
        test!(u32, 5);
        test!(u32, 0);

        test!(u32, std::i8::MAX as u32);

        test!(u32, std::u8::MIN as u32);
        test!(u32, std::u8::MAX as u32);

        test!(u32, std::i16::MIN as u32);
        test!(u32, std::i16::MAX as u32);

        test!(u32, std::u16::MIN as u32);
        test!(u32, std::u16::MAX as u32);

        test!(u32, std::i32::MIN as u32);
        test!(u32, std::i32::MAX as u32);

        test!(u32, std::u32::MIN as u32);
        test!(u32, std::u32::MAX as u32);
    }

    #[test]
    fn encode_decode_i64() {
        test!(i64, 5);
        test!(i64, 0);

        test!(i64, std::i8::MAX as i64);

        test!(i64, std::u8::MIN as i64);
        test!(i64, std::u8::MAX as i64);

        test!(i64, std::i16::MIN as i64);
        test!(i64, std::i16::MAX as i64);

        test!(i64, std::u16::MIN as i64);
        test!(i64, std::u16::MAX as i64);

        test!(i64, std::i32::MIN as i64);
        test!(i64, std::i32::MAX as i64);

        test!(i64, std::u32::MIN as i64);
        test!(i64, std::u32::MAX as i64);

        test!(i64, std::i64::MIN as i64);
        test!(i64, std::i64::MAX as i64);
    }

    #[test]
    fn encode_decode_u64() {
        test!(u64, 5);
        test!(u64, 0);

        test!(u64, std::u8::MIN as u64);
        test!(u64, std::u8::MAX as u64);

        test!(u64, std::u16::MIN as u64);
        test!(u64, std::u16::MAX as u64);

        test!(u64, std::u32::MIN as u64);
        test!(u64, std::u32::MAX as u64);

        test!(u64, std::u64::MAX as u64);
        test!(u64, std::u64::MAX as u64);
    }

    #[test]
    fn encode_decode_addr() {
        let addr: Address = [0x10; Address::len()].into();

        test!(Address, addr);
    }

    #[test]
    fn display_addr() {
        let bytes = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88, 0x99, 0xAA,
        ];

        let addr: Address = bytes.into();
        let s = format!("{}", addr);
        assert_eq!(s, "102030405060708090a0112233445566778899aa");
    }
}
