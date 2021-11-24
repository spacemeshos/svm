use svm_sdk_mock::template;

#[template]
mod Template {
    #[fundable_hook]
    #[endpoint]
    fn do_nothing() -> u8 {
        0
    }
}

fn main() {}
