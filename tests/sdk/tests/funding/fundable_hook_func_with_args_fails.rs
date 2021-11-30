use svm_sdk_mock::template;

#[template]
mod Template {
    #[fundable_hook]
    fn deny(v: svm_sdk::Amount) {}
}

fn main() {}
