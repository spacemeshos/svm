use svm_sdk::template;

#[template]
mod Template {
    #[ctor]
    #[fundable(deny)]
    fn init() {}
}

fn main() {}
