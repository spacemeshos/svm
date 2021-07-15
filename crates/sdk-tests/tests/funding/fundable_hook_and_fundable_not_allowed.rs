use svm_sdk::template;

#[template]
mod Template {
    #[fundable_hook]
    #[fundable(default)]
    fn get() {}
}

fn main() {}
