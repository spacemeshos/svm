#![allow(unused)]

use svm_sdk::{value::AddressOwned, Amount};
use svm_sdk_macros::AppStorage;

#[derive(AppStorage, Debug)]
struct Test {
    flag: bool,
    amount: Amount,
    addr: AddressOwned,
    addrs: [AddressOwned; 3],
    ubyte: u8,
    sbyte: i8,
    uword: u16,
    sword: i16,
    udoubleword: u32,
    sdoubleword: i32,
    uquadword: u64,
    squadword: i64,
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
    assert_eq!(TestStorage::get_ubyte(), 0);

    TestStorage::set_ubyte(255u8);

    assert_eq!(TestStorage::get_ubyte(), 255u8);
}

#[test]
fn test_i8() {
    assert_eq!(TestStorage::get_sbyte(), 0);

    TestStorage::set_sbyte(127i8);

    assert_eq!(TestStorage::get_sbyte(), 127i8);
}

#[test]
fn test_u16() {
    assert_eq!(TestStorage::get_uword(), 0);

    TestStorage::set_uword(std::u16::MAX);

    assert_eq!(TestStorage::get_uword(), std::u16::MAX);
}

#[test]
fn test_i16() {
    assert_eq!(TestStorage::get_sword(), 0);

    TestStorage::set_sword(std::i16::MAX);

    assert_eq!(TestStorage::get_sword(), std::i16::MAX);
}

#[test]
fn test_u32() {
    assert_eq!(TestStorage::get_udoubleword(), 0);

    TestStorage::set_udoubleword(std::u32::MAX);

    assert_eq!(TestStorage::get_udoubleword(), std::u32::MAX);
}

#[test]
fn test_i32() {
    assert_eq!(TestStorage::get_sdoubleword(), 0);

    TestStorage::set_sdoubleword(std::i32::MAX);

    assert_eq!(TestStorage::get_sdoubleword(), std::i32::MAX);
}

#[test]
fn test_u64() {
    assert_eq!(TestStorage::get_uquadword(), 0);

    TestStorage::set_uquadword(std::u64::MAX);

    assert_eq!(TestStorage::get_uquadword(), std::u64::MAX);
}

#[test]
fn test_i64() {
    assert_eq!(TestStorage::get_squadword(), 0);

    TestStorage::set_squadword(std::i64::MAX);

    assert_eq!(TestStorage::get_squadword(), std::i64::MAX);
}

#[test]
fn test_address_owned() {
    assert_eq!(TestStorage::get_addr(), AddressOwned([0; 20]));

    let addr = AddressOwned([0x10; 20]);
    TestStorage::set_addr(&addr);

    assert_eq!(TestStorage::get_addr(), AddressOwned([0x10; 20]));
}
