use svm_sdk::template;

#[template]
mod Template {
    #[fundable_hook]
    fn deny() -> u32 {
        0
    }
}

fn main() {}
