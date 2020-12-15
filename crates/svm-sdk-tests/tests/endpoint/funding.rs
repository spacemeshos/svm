use svm_sdk::{app, Amount};

use svm_sdk_tests::call_and_fund_1;

#[app]
mod App {
    #[storage]
    struct Storage {
        coins: Amount,
    }

    #[fundable(update_coins)]
    #[endpoint]
    fn do_nothing() {}

    #[fundable_hook]
    fn update_coins(value: Amount) {
        let old_coins = Storage::get_coins();
        let new_coins = old_coins + value;

        Storage::set_coins(new_coins);
    }
}

fn test_fund() {
    let params = Vec::<bool>::new();
    let value = Amount(10);

    let coins = Storage::get_coins();
    assert_eq!(coins, Amount(0));

    let res: () = call_and_fund_1(do_nothing, params, value);
    assert_eq!(res, ());

    let coins = Storage::get_coins();
    assert_eq!(coins, value);
}

fn main() {
    test_fund();
}
