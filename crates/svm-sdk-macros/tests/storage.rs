#![allow(unused)]

use svm_sdk::{value::Address, Amount};
use svm_sdk_macros::AppStorage;

#[derive(AppStorage, Debug)]
struct Test {
    debug_mode: bool,
    total: Amount,
    // total_fqn: svm_sdk::Amount,
}

#[test]
fn test_bool() {
    assert_eq!(TestStorage::get_debug_mode(), false);

    TestStorage::set_debug_mode(true);

    assert_eq!(TestStorage::get_debug_mode(), true);
}

#[test]
fn test_amount() {
    assert_eq!(TestStorage::get_total(), Amount(0));

    TestStorage::set_total(Amount(10));

    assert_eq!(TestStorage::get_total(), Amount(10));
}

// #[test]
// fn test_amount_full_qualified_name() {
//     assert_eq!(TestStorage::get_total_fqn(), svm_sdk::Amount(0));

//     TestStorage::set_total_fqn(svm_sdk::Amount(10));

//     assert_eq!(TestStorage::get_total_fqn(), svm_sdk::Amount(10));
// }
