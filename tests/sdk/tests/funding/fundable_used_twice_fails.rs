use svm_sdk::template;

#[template]
mod Template {
    #[fundable(allow)]
    #[fundable(allow)]
    #[endpoint]
    fn get(value: svm_sdk::Amount) {}
}

fn main() {}
