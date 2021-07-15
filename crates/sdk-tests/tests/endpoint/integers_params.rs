use svm_sdk::template;

use svm_sdk_tests::call_1;

#[template]
mod Template {
    #[endpoint]
    fn add_u8(a: u8, b: u8) -> u8 {
        a + b
    }

    #[endpoint]
    fn add_i8(a: i8, b: i8) -> i8 {
        a + b
    }

    #[endpoint]
    fn add_u16(a: u16, b: u16) -> u16 {
        a + b
    }

    #[endpoint]
    fn add_i16(a: i16, b: i16) -> i16 {
        a + b
    }

    #[endpoint]
    fn add_u32(a: u32, b: u32) -> u32 {
        a + b
    }

    #[endpoint]
    fn add_i32(a: i32, b: i32) -> i32 {
        a + b
    }

    #[endpoint]
    fn add_u64(a: u64, b: u64) -> u64 {
        a + b
    }

    #[endpoint]
    fn add_i64(a: i64, b: i64) -> i64 {
        a + b
    }
}

fn test_u8() {
    let res: u8 = call_1(add_u8, vec![2u8, 3u8]);

    assert_eq!(res, 5u8);
}

fn test_i8() {
    let res: i8 = call_1(add_i8, vec![-2i8, -3i8]);

    assert_eq!(res, -5i8);
}

fn test_u16() {
    let res: u16 = call_1(add_u16, vec![2u16, 3u16]);

    assert_eq!(res, 5u16);
}

fn test_i16() {
    let res: i16 = call_1(add_i16, vec![-2i16, -3i16]);

    assert_eq!(res, -5i16);
}

fn test_u32() {
    let res: u32 = call_1(add_u32, vec![2u32, 3u32]);

    assert_eq!(res, 5u32);
}

fn test_i32() {
    let res: i32 = call_1(add_i32, vec![-2i32, -3i32]);

    assert_eq!(res, -5i32);
}

fn test_u64() {
    let res: u64 = call_1(add_u64, vec![2u64, 3u64]);

    assert_eq!(res, 5u64);
}

fn test_i64() {
    let res: i64 = call_1(add_i64, vec![-2i64, -3i64]);

    assert_eq!(res, -5i64);
}

fn main() {
    test_u8();
    test_i8();

    test_u16();
    test_i16();

    test_u32();
    test_i32();

    test_u64();
    test_i64();
}
