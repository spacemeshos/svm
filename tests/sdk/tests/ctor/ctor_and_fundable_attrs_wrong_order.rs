use svm_sdk_mock::template;

#[template]
mod Template {
    #[ctor]
    #[fundable(deny)]
    fn init() {}
}

fn main() {}
