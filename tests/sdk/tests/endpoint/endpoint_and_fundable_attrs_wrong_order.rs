use svm_sdk_mock::template;

#[template]
mod Template {
    #[endpoint]
    #[fundable(deny)]
    fn init() {}
}

fn main() {}
