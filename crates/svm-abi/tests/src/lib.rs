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
    use svm_sdk::value::{Address, AddressOwned, Composite, Primitive, Value};

    macro_rules! test_primitive {
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

    macro_rules! test_array {
        ($ty:ty, $rust_array:expr) => {{
            let mut bytes = Vec::new();

            $rust_array.to_vec().encode(&mut bytes);

            let mut cursor = Cursor::new(&bytes);
            let decoder = Decoder::new();

            let value: Value = decoder.decode_value(&mut cursor).unwrap();
            let decoded: $ty = value.into();

            assert_eq!(decoded, $rust_array);
        }};
    }

    #[test]
    fn owned_addr_deref() {
        let bytes: [u8; 20] = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        ];

        let owned = AddressOwned(bytes);
        let borrowed = owned.deref();

        assert_eq!(borrowed.0, &bytes);
    }

    #[test]
    fn encode_decode_bool() {
        test_primitive!(bool, true);
        test_primitive!(bool, false);

        test_array!([bool; 1], [true]);
        test_array!([bool; 2], [true, false]);
        test_array!([bool; 3], [true, false, true]);
    }

    #[test]
    fn encode_decode_i8() {
        test_primitive!(i8, -5);
        test_primitive!(i8, 5);

        test_primitive!(i8, 0);
        test_primitive!(i8, -1);

        test_primitive!(i8, std::i8::MIN as i8);
        test_primitive!(i8, std::i8::MAX as i8);

        test_array!([i8; 1], [10i8]);
        test_array!([i8; 2], [-5i8, 5i8]);
        test_array!([i8; 3], [10i8, 20i8, 30i8]);
    }

    #[test]
    fn encode_decode_u8() {
        test_primitive!(u8, 5);
        test_primitive!(u8, 0);

        test_primitive!(u8, std::u8::MIN as u8);
        test_primitive!(u8, std::u8::MAX as u8);

        test_array!([u8; 1], [10u8]);
        test_array!([u8; 2], [5u8, 10u8]);
        test_array!([u8; 3], [10u8, 20u8, 30u8]);
    }

    #[test]
    fn encode_decode_i16() {
        test_primitive!(i16, -5);
        test_primitive!(i16, 5);

        test_primitive!(i16, 0);
        test_primitive!(i16, -1);

        test_primitive!(i16, std::i8::MIN as i16);
        test_primitive!(i16, std::i8::MAX as i16);

        test_primitive!(i16, std::u8::MIN as i16);
        test_primitive!(i16, std::u8::MAX as i16);

        test_primitive!(i16, std::i16::MIN as i16);
        test_primitive!(i16, std::i16::MAX as i16);

        test_array!([i16; 1], [10i16]);
        test_array!([i16; 2], [-5i16, 5i16]);
        test_array!([i16; 3], [10i16, 20i16, 30i16]);
    }

    #[test]
    fn encode_decode_u16() {
        test_primitive!(u16, 5);
        test_primitive!(u16, 0);

        test_primitive!(u16, 127);

        test_primitive!(u16, std::i8::MIN as u16);
        test_primitive!(u16, std::i8::MAX as u16);

        test_primitive!(u16, std::u8::MIN as u16);
        test_primitive!(u16, std::u8::MAX as u16);

        test_primitive!(u16, std::i16::MAX as u16);
        test_primitive!(u16, std::u16::MAX as u16);

        test_array!([u16; 1], [10u16]);
        test_array!([u16; 2], [5u16, 10u16]);
        test_array!([u16; 3], [10u16, 20u16, 30u16]);
    }

    #[test]
    fn encode_decode_i32() {
        test_primitive!(i32, 5);
        test_primitive!(i32, 0);
        test_primitive!(i32, -1);

        test_primitive!(i32, std::i8::MIN as i32);
        test_primitive!(i32, std::i8::MAX as i32);

        test_primitive!(i32, std::u8::MIN as i32);
        test_primitive!(i32, std::u8::MAX as i32);

        test_primitive!(i32, std::i16::MIN as i32);
        test_primitive!(i32, std::i16::MAX as i32);

        test_primitive!(i32, std::u16::MIN as i32);
        test_primitive!(i32, std::u16::MAX as i32);

        test_primitive!(i32, std::i32::MIN as i32);
        test_primitive!(i32, std::i32::MAX as i32);

        test_array!([i32; 1], [10i32]);
        test_array!([i32; 2], [-5i32, 5i32]);
        test_array!([i32; 3], [10i32, 20i32, 30i32]);
    }

    #[test]
    fn encode_decode_u32() {
        test_primitive!(u32, 5);
        test_primitive!(u32, 0);

        test_primitive!(u32, std::i8::MAX as u32);

        test_primitive!(u32, std::u8::MIN as u32);
        test_primitive!(u32, std::u8::MAX as u32);

        test_primitive!(u32, std::i16::MIN as u32);
        test_primitive!(u32, std::i16::MAX as u32);

        test_primitive!(u32, std::u16::MIN as u32);
        test_primitive!(u32, std::u16::MAX as u32);

        test_primitive!(u32, std::i32::MIN as u32);
        test_primitive!(u32, std::i32::MAX as u32);

        test_primitive!(u32, std::u32::MIN as u32);
        test_primitive!(u32, std::u32::MAX as u32);

        test_array!([u32; 1], [10u32]);
        test_array!([u32; 2], [5u32, 10u32]);
        test_array!([u32; 3], [1032, 20u32, 30u32]);
    }

    #[test]
    fn encode_decode_i64() {
        test_primitive!(i64, 5);
        test_primitive!(i64, 0);

        test_primitive!(i64, std::i8::MAX as i64);

        test_primitive!(i64, std::u8::MIN as i64);
        test_primitive!(i64, std::u8::MAX as i64);

        test_primitive!(i64, std::i16::MIN as i64);
        test_primitive!(i64, std::i16::MAX as i64);

        test_primitive!(i64, std::u16::MIN as i64);
        test_primitive!(i64, std::u16::MAX as i64);

        test_primitive!(i64, std::i32::MIN as i64);
        test_primitive!(i64, std::i32::MAX as i64);

        test_primitive!(i64, std::u32::MIN as i64);
        test_primitive!(i64, std::u32::MAX as i64);

        test_primitive!(i64, std::i64::MIN as i64);
        test_primitive!(i64, std::i64::MAX as i64);

        test_array!([i64; 1], [10i64]);
        test_array!([i64; 2], [-5i64, 5i64]);
        test_array!([i64; 3], [10i64, 20i64, 30i64]);
    }

    #[test]
    fn encode_decode_u64() {
        test_primitive!(u64, 5);
        test_primitive!(u64, 0);

        test_primitive!(u64, std::u8::MIN as u64);
        test_primitive!(u64, std::u8::MAX as u64);

        test_primitive!(u64, std::u16::MIN as u64);
        test_primitive!(u64, std::u16::MAX as u64);

        test_primitive!(u64, std::u32::MIN as u64);
        test_primitive!(u64, std::u32::MAX as u64);

        test_primitive!(u64, std::u64::MAX as u64);
        test_primitive!(u64, std::u64::MAX as u64);

        test_array!([u64; 1], [10u64]);
        test_array!([u64; 2], [5u64, 10u64]);
        test_array!([u64; 3], [1064, 20u64, 30u64]);
    }

    #[test]
    fn encode_decode_addr() {
        test_primitive!(Address, Address(&[0x10; 20]));

        test_primitive!(AddressOwned, AddressOwned([0x10; 20]));

        test_array!([AddressOwned; 1], [AddressOwned([0x10; 20])]);
        test_array!(
            [AddressOwned; 2],
            [AddressOwned([0x10; 20]), AddressOwned([0x20; 20])]
        );
    }

    #[test]
    fn display_addr() {
        let bytes = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88, 0x99, 0xAA,
        ];

        let addr = Address(&bytes);
        let s = format!("{}", addr);
        assert_eq!(s, "102030405060708090a0112233445566778899aa");

        let addr = AddressOwned(bytes);
        let s = format!("{}", addr);
        assert_eq!(s, "102030405060708090a0112233445566778899aa");
    }
}
