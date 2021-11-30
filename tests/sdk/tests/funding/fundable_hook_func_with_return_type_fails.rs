use svm_sdk_mock::template;

#[template]
mod Template {
    #[fundable_hook]
    fn deny() -> u32 {
        0
    }
}

fn main() {}
