//! This crate tests the encoding & decoding of a function buffer.
//! using SVM default ABI.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

#[cfg(test)]
mod tests {
    use svm_abi_decoder::CallData;
    use svm_abi_encoder::ABIEncoder;

    use svm_sdk_std::{Option, Vec};

    use svm_sdk_types::value::Value;
    use svm_sdk_types::{Address, Amount};

    macro_rules! as_static {
        ($bytes:expr) => {
            unsafe { core::mem::transmute::<_, &'static [u8]>(&$bytes[..]) }
        };
    }

    macro_rules! test_primitive {
        ($ty:ty, $rust_value:expr) => {{
            let rust_value: $ty = $rust_value.clone();
            let value: Value = rust_value.into();

            let mut buf_rust = Vec::with_capacity(1000);
            let mut buf_value = Vec::with_capacity(1000);

            let rust_value: $ty = $rust_value.clone();
            rust_value.encode(&mut buf_rust);
            value.encode(&mut buf_value);

            // Asserting that encoding a Rust primitive
            // gives the same results when encoding the corresponding `Value` wrapper.
            assert_eq!(&buf_rust, &buf_value);

            let mut calldata = CallData::new(as_static!(&buf_rust));
            let decoded_val: Value = calldata.next().unwrap();

            let decoded_rust_val: $ty = decoded_val.into();
            assert_eq!(decoded_rust_val, $rust_value);
        }};
    }

    macro_rules! test_array {
        ($ty:ty, $rust_array:expr) => {{
            let mut bytes = Vec::with_capacity(1000);

            $rust_array.encode(&mut bytes);

            let mut calldata = CallData::new(as_static!(&bytes));
            let value: Value = calldata.next().unwrap();
            let decoded: $ty = value.into();

            assert_eq!(decoded, $rust_array);
        }};
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
    fn encode_decode_none() {
        test_primitive!(Option<u8>, Option::None);
        test_primitive!(Option<u32>, Option::None);
    }

    #[test]
    fn encode_decode_unit() {
        test_primitive!((), ());
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
    fn encode_decode_amount() {
        test_primitive!(Amount, Amount(5));
        test_primitive!(Amount, Amount(0));

        test_primitive!(Amount, Amount(std::u8::MIN as u64));
        test_primitive!(Amount, Amount(std::u8::MAX as u64));

        test_primitive!(Amount, Amount(std::u16::MIN as u64));
        test_primitive!(Amount, Amount(std::u16::MAX as u64));

        test_primitive!(Amount, Amount(std::u32::MIN as u64));
        test_primitive!(Amount, Amount(std::u32::MAX as u64));

        test_primitive!(Amount, Amount(std::u64::MAX as u64));
        test_primitive!(Amount, Amount(std::u64::MAX as u64));

        test_array!([Amount; 1], [Amount(10)]);
        test_array!([Amount; 2], [Amount(5), Amount(10)]);
        test_array!([Amount; 3], [Amount(10), Amount(20), Amount(30)]);
    }

    #[test]
    fn encode_decode_addr() {
        let addr: Address = Address::repeat(0x10);

        test_primitive!(Address, addr);
    }

    #[test]
    fn calldata_next() {
        let a: u32 = 10;

        let mut buf = Vec::with_capacity(1000);
        a.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let a_ = calldata.next().unwrap().into();

        assert_eq!(a, a_);
    }

    #[test]
    fn calldata_next_1() {
        let a: u32 = 10;

        let mut buf = Vec::with_capacity(1000);
        a.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let a_ = calldata.next_1();

        assert_eq!(a, a_);
    }

    #[test]
    fn calldata_next_2() {
        let a: u32 = 10;
        let b: i16 = 20;

        let mut buf = Vec::with_capacity(1000);

        a.encode(&mut buf);
        b.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let (a_, b_) = calldata.next_2();

        assert_eq!(a, a_);
        assert_eq!(b, b_);
    }

    #[test]
    fn calldata_next_3() {
        let a: u32 = 10;
        let b: i16 = 20;
        let c = true;

        let mut buf = Vec::with_capacity(1000);

        a.encode(&mut buf);
        b.encode(&mut buf);
        c.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let (a_, b_, c_) = calldata.next_3();

        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
    }

    #[test]
    fn calldata_next_4() {
        let a: u32 = 10;
        let b: i16 = 20;
        let c = true;
        let d: [u8; 2] = [30, 40];

        let mut buf = Vec::with_capacity(1000);

        a.encode(&mut buf);
        b.encode(&mut buf);
        c.encode(&mut buf);
        d.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let (a_, b_, c_, d_): (u32, i16, bool, [u8; 2]) = calldata.next_4();

        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
        assert_eq!(d, d_);
    }

    #[test]
    fn calldata_next_5() {
        let a: u32 = 10;
        let b: i16 = 20;
        let c = true;
        let d: [u8; 2] = [30, 40];
        let e: [u16; 3] = [50, 60, 70];

        let mut buf = Vec::with_capacity(1000);

        a.encode(&mut buf);
        b.encode(&mut buf);
        c.encode(&mut buf);
        d.encode(&mut buf);
        e.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let (a_, b_, c_, d_, e_): (u32, i16, bool, [u8; 2], [u16; 3]) = calldata.next_5();

        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
        assert_eq!(d, d_);
        assert_eq!(e, e_);
    }

    #[test]
    fn calldata_next_6() {
        let a: u32 = 10;
        let b: i16 = 20;
        let c = true;
        let d: [u8; 2] = [30, 40];
        let e: [u16; 3] = [50, 60, 70];
        let f = Amount(100);

        let mut buf = Vec::with_capacity(1000);

        a.encode(&mut buf);
        b.encode(&mut buf);
        c.encode(&mut buf);
        d.encode(&mut buf);
        e.encode(&mut buf);
        f.encode(&mut buf);

        let mut calldata = CallData::new(as_static!(buf));
        let (a_, b_, c_, d_, e_, f_): (u32, i16, bool, [u8; 2], [u16; 3], Amount) =
            calldata.next_6();

        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
        assert_eq!(d, d_);
        assert_eq!(e, e_);
        assert_eq!(f, f_);
    }

    #[test]
    fn display_addr() {
        let bytes = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88, 0x99, 0xAA,
        ];

        let addr: Address = bytes.into();
        let fmt = format!("{:?}", addr);
        assert_eq!(fmt, "102030405060708090a0112233445566778899aa");
    }
}
