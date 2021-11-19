use svm_sdk_mock::template;

#[template]
mod Template {
    #[fundable(allow)]
    #[fundable(allow)]
    #[endpoint]
    fn get(value: svm_sdk::Amount) {}
}

fn main() {}
