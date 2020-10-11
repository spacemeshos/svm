#![allow(unused)]

use svm_sdk::{value::Address, Amount};
use svm_sdk_macros::AppStorage;

#[derive(AppStorage, Debug)]
struct Test {
    debug_mode: bool,
}

#[test]
fn test_bool() {
    assert_eq!(TestStorage::get_debug_mode(), false);

    TestStorage::set_debug_mode(true);

    assert_eq!(TestStorage::get_debug_mode(), true);
}
