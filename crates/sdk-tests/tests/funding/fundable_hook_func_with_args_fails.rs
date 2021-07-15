use svm_sdk::template;

#[template]
mod Template {
    #[fundable_hook]
    fn deny(v: svm_sdk::Amount) {}
}

fn main() {}
