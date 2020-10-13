#![allow(unused)]

use svm_sdk::{value::AddressOwned, Amount};
use svm_sdk_macros::AppStorage;

#[derive(AppStorage, Debug)]
struct Test {
    flag: bool,
    amount: Amount,
    addr: AddressOwned,
    addrs: [AddressOwned; 3],
    uu8: u8,
    ii8: i8,
    uu16: u16,
    ii16: i16,
    uu32: u32,
    ii32: i32,
    uu64: u64,
    ii64: i64,
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
fn test_address_owned() {
    assert_eq!(TestStorage::get_addr(), AddressOwned([0; 20]));

    let addr = AddressOwned([0x10; 20]);
    TestStorage::set_addr(&addr);

    assert_eq!(TestStorage::get_addr(), AddressOwned([0x10; 20]));
}
