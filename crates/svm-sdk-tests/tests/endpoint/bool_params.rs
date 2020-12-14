use svm_sdk::app;

use svm_sdk_tests::call_1;

#[app]
mod App {
    #[endpoint]
    fn and(a: bool, b: bool) -> bool {
        a && b
    }
}

fn main() {
    let res: bool = call_1(and, vec![true, true]);
    assert_eq!(res, true);

    let res: bool = call_1(and, vec![false, true]);
    assert_eq!(res, false);
}
