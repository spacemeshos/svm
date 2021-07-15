use svm_sdk::template;

#[template]
mod Template {
    #[fundable_hook]
    #[endpoint]
    fn do_nothing() -> u8 {
        0
    }
}

fn main() {}
