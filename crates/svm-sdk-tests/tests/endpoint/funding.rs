use svm_sdk::host::MockHost;
use svm_sdk::{app, Amount};

use svm_sdk_tests::call_and_fund_1;

#[app]
mod App {
    // #[fundable(take_coins)]
    #[endpoint]
    fn do_something() -> u8 {
        0
    }

    #[before_fund]
    fn take_coins(_value: Amount) {
        Node
    }
}

fn test_fund() {
    let params = Vec::<bool>::new();
    let value = Amount(10);

    let res: u8 = call_and_fund_1(do_something, params, value);
    assert_eq!(res, 0);
}

fn main() {
    test_fund();
}
