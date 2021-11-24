use svm_sdk_mock::template;

#[template]
mod Template {
    #[fundable_hook]
    #[fundable(default)]
    fn get() {}
}

fn main() {}
