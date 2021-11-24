use svm_sdk_mock::{template, Amount};

use svm_sdk_tests::{call_1, call_2};

#[template]
mod Template {
    #[endpoint]
    fn add(a: Amount, b: Amount) -> Amount {
        a + b
    }

    #[endpoint]
    fn mul(a: svm_sdk::Amount, b: svm_sdk::Amount) -> Amount {
        a * b
    }

    #[endpoint]
    fn swap(a: Amount, b: Amount) -> (Amount, Amount) {
        (b, a)
    }
}

fn test_add(a: u64, b: u64) {
    let a = Amount(a);
    let b = Amount(b);

    let res: Amount = call_1(add, vec![a, b]);
    assert_eq!(res, a + b);
}

fn test_mul(a: u64, b: u64) {
    let a = Amount(a);
    let b = Amount(b);

    let res: Amount = call_1(mul, vec![a, b]);
    assert_eq!(res, a * b);
}

fn test_swap(a: u64, b: u64) {
    let a = Amount(a);
    let b = Amount(b);

    let res: (Amount, Amount) = call_2(swap, vec![a, b]);
    assert_eq!(res, (b, a));
}

fn main() {
    let a = 2;
    let b = 3;

    test_add(a, b);
    test_mul(a, b);
    test_swap(a, b);
}
