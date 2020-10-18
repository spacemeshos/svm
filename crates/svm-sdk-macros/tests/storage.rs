#![allow(unused)]

use svm_sdk::{Address, Amount};
use svm_sdk_macros::storage;

#[storage]
#[derive(Debug)]
struct Test {
    // Primitives
    flag: bool,
    amount: Amount,
    addr: Address,
    uu8: u8,
    ii8: i8,
    uu16: u16,
    ii16: i16,
    uu32: u32,
    ii32: i32,
    uu64: u64,
    ii64: i64,

    // Arrays
    flags: [bool; 2],
    amounts: [Amount; 3],
    addrs: [Address; 2],
    uu8s: [u8; 2],
    ii8s: [i8; 2],
    uu16s: [u16; 2],
    ii16s: [i16; 2],
    uu32s: [u32; 2],
    ii32s: [i32; 2],
    uu64s: [u64; 2],
    ii64s: [i64; 2],
}

#[test]
fn test_bool() {
    assert_eq!(TestStorage::get_flag(), false);

    TestStorage::set_flag(true);

    assert_eq!(TestStorage::get_flag(), true);
}

#[test]
fn test_amount() {
    assert_eq!(TestStorage::get_amount(), Amount(0));

    TestStorage::set_amount(Amount(10));

    assert_eq!(TestStorage::get_amount(), Amount(10));
}

#[test]
fn test_u8() {
    assert_eq!(TestStorage::get_uu8(), 0);

    TestStorage::set_uu8(255u8);

    assert_eq!(TestStorage::get_uu8(), 255u8);
}

#[test]
fn test_i8() {
    assert_eq!(TestStorage::get_ii8(), 0);

    TestStorage::set_ii8(127i8);

    assert_eq!(TestStorage::get_ii8(), 127i8);
}

#[test]
fn test_u16() {
    assert_eq!(TestStorage::get_uu16(), 0);

    TestStorage::set_uu16(std::u16::MAX);

    assert_eq!(TestStorage::get_uu16(), std::u16::MAX);
}

#[test]
fn test_i16() {
    assert_eq!(TestStorage::get_ii16(), 0);

    TestStorage::set_ii16(std::i16::MAX);

    assert_eq!(TestStorage::get_ii16(), std::i16::MAX);
}

#[test]
fn test_u32() {
    assert_eq!(TestStorage::get_uu32(), 0);

    TestStorage::set_uu32(std::u32::MAX);

    assert_eq!(TestStorage::get_uu32(), std::u32::MAX);
}

#[test]
fn test_i32() {
    assert_eq!(TestStorage::get_ii32(), 0);

    TestStorage::set_ii32(std::i32::MAX);

    assert_eq!(TestStorage::get_ii32(), std::i32::MAX);
}

#[test]
fn test_u64() {
    assert_eq!(TestStorage::get_uu64(), 0);

    TestStorage::set_uu64(std::u64::MAX);

    assert_eq!(TestStorage::get_uu64(), std::u64::MAX);
}

#[test]
fn test_i64() {
    assert_eq!(TestStorage::get_ii64(), 0);

    TestStorage::set_ii64(std::i64::MAX);

    assert_eq!(TestStorage::get_ii64(), std::i64::MAX);
}

#[test]
fn test_address() {
    let empty: Address = [0; 20].into();

    assert_eq!(TestStorage::get_addr(), empty);

    let addr: Address = [0x10; 20].into();
    TestStorage::set_addr(&addr);

    assert_eq!(TestStorage::get_addr(), addr);
}

#[test]
fn test_array_bool() {
    assert_eq!(TestStorage::get_flags(0), false);
    assert_eq!(TestStorage::get_flags(1), false);

    TestStorage::set_flags(0, true);
    assert_eq!(TestStorage::get_flags(0), true);
    assert_eq!(TestStorage::get_flags(1), false);

    TestStorage::set_flags(1, true);
    assert_eq!(TestStorage::get_flags(0), true);
    assert_eq!(TestStorage::get_flags(1), true);
}

#[test]
fn test_array_amount() {
    assert_eq!(TestStorage::get_amounts(0), Amount(0));
    assert_eq!(TestStorage::get_amounts(1), Amount(0));
    assert_eq!(TestStorage::get_amounts(2), Amount(0));

    TestStorage::set_amounts(0, Amount(10));
    TestStorage::set_amounts(1, Amount(20));
    TestStorage::set_amounts(2, Amount(30));

    assert_eq!(TestStorage::get_amounts(0), Amount(10));
    assert_eq!(TestStorage::get_amounts(1), Amount(20));
    assert_eq!(TestStorage::get_amounts(2), Amount(30));
}

#[test]
fn test_array_address() {
    let empty: Address = [0; Address::len()].into();

    assert_eq!(TestStorage::get_addrs(0), empty);
    assert_eq!(TestStorage::get_addrs(1), empty);

    let addr: Address = [0x10; 20].into();
    TestStorage::set_addrs(0, &addr);

    assert_eq!(TestStorage::get_addrs(0), addr);
    assert_eq!(TestStorage::get_addrs(1), empty);
}

#[test]
fn test_array_u8() {
    assert_eq!(TestStorage::get_uu8s(0), 0u8);
    assert_eq!(TestStorage::get_uu8s(1), 0u8);

    TestStorage::set_uu8s(0, 10u8);

    assert_eq!(TestStorage::get_uu8s(0), 10u8);
    assert_eq!(TestStorage::get_uu8s(1), 0u8);
}

#[test]
fn test_array_i8() {
    assert_eq!(TestStorage::get_ii8s(0), 0i8);
    assert_eq!(TestStorage::get_ii8s(1), 0i8);

    TestStorage::set_ii8s(0, -10i8);

    assert_eq!(TestStorage::get_ii8s(0), -10i8);
    assert_eq!(TestStorage::get_ii8s(1), 0i8);
}

#[test]
fn test_array_u16() {
    assert_eq!(TestStorage::get_uu16s(0), 0u16);
    assert_eq!(TestStorage::get_uu16s(1), 0u16);

    TestStorage::set_uu16s(0, 10u16);

    assert_eq!(TestStorage::get_uu16s(0), 10u16);
    assert_eq!(TestStorage::get_uu16s(1), 0u16);
}

#[test]
fn test_array_i16() {
    assert_eq!(TestStorage::get_ii16s(0), 0i16);
    assert_eq!(TestStorage::get_ii16s(1), 0i16);

    TestStorage::set_ii16s(0, -10i16);

    assert_eq!(TestStorage::get_ii16s(0), -10i16);
    assert_eq!(TestStorage::get_ii16s(1), 0i16);
}

#[test]
fn test_array_u32() {
    assert_eq!(TestStorage::get_uu32s(0), 0u32);
    assert_eq!(TestStorage::get_uu32s(1), 0u32);

    TestStorage::set_uu32s(0, 10u32);

    assert_eq!(TestStorage::get_uu32s(0), 10u32);
    assert_eq!(TestStorage::get_uu32s(1), 0u32);
}

#[test]
fn test_array_i32() {
    assert_eq!(TestStorage::get_ii32s(0), 0i32);
    assert_eq!(TestStorage::get_ii32s(1), 0i32);

    TestStorage::set_ii32s(0, -10i32);

    assert_eq!(TestStorage::get_ii32s(0), -10i32);
    assert_eq!(TestStorage::get_ii32s(1), 0i32);
}

#[test]
fn test_array_u64() {
    assert_eq!(TestStorage::get_uu64s(0), 0u64);
    assert_eq!(TestStorage::get_uu64s(1), 0u64);

    TestStorage::set_uu64s(0, 10u64);

    assert_eq!(TestStorage::get_uu64s(0), 10u64);
    assert_eq!(TestStorage::get_uu64s(1), 0u64);
}

#[test]
fn test_array_i64() {
    assert_eq!(TestStorage::get_ii64s(0), 0i64);
    assert_eq!(TestStorage::get_ii64s(1), 0i64);

    TestStorage::set_ii64s(0, -10i64);

    assert_eq!(TestStorage::get_ii64s(0), -10i64);
    assert_eq!(TestStorage::get_ii64s(1), 0i64);
}
