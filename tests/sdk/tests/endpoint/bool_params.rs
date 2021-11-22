use svm_sdk_mock::template;

use svm_sdk_tests::{call_1, call_2};

#[template]
mod Template {
    #[endpoint]
    fn not(a: bool) -> bool {
        !(a)
    }

    #[endpoint]
    fn and(a: bool, b: bool) -> bool {
        a && b
    }

    #[endpoint]
    fn swap(a: bool, b: bool) -> (bool, bool) {
        (b, a)
    }
}

fn test_not() {
    let res: bool = call_1(not, vec![false]);
    assert_eq!(res, true);

    let res: bool = call_1(not, vec![true]);
    assert_eq!(res, false);
}

fn test_and() {
    let res: bool = call_1(and, vec![true, true]);
    assert_eq!(res, true);

    let res: bool = call_1(and, vec![false, true]);
    assert_eq!(res, false);
}

fn test_swap() {
    let res: (bool, bool) = call_2(swap, vec![true, false]);
    assert_eq!(res, (false, true));
}

fn main() {
    test_not();
    test_and();
    test_swap();
}
