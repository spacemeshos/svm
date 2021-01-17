use svm_sdk::storage::MockStorage;
use svm_sdk::{app, Amount};

use svm_sdk_tests::call_and_fund_1;

#[app]
mod App {
    #[storage]
    struct Storage {
        coins: Amount,

        called: bool,
    }

    #[fundable(update_coins)]
    #[endpoint]
    fn do_nothing() {}

    #[fundable_hook]
    fn update_coins() {
        let value = Node::value();

        let old_coins = Storage::get_coins();
        let new_coins = old_coins + value;

        Storage::set_called(true);
        Storage::set_coins(new_coins);
    }
}

fn test_fund_zero() {
    MockStorage::clear();

    let params = Vec::<bool>::new();
    let value = Amount(0);

    let called = Storage::get_called();
    assert!(!called);

    let res: () = call_and_fund_1(do_nothing, params, value);
    assert_eq!(res, ());

    let called = Storage::get_called();
    assert!(!called);
}

fn test_fund_positive() {
    MockStorage::clear();

    let params = Vec::<bool>::new();
    let value = Amount(10);

    let coins = Storage::get_coins();
    assert_eq!(coins, Amount(0));

    let called = Storage::get_called();
    assert!(!called);

    let res: () = call_and_fund_1(do_nothing, params, value);
    assert_eq!(res, ());

    let coins = Storage::get_coins();
    assert_eq!(coins, value);

    let called = Storage::get_called();
    assert!(called);
}

fn main() {
    test_fund_zero();
    test_fund_positive();
}
