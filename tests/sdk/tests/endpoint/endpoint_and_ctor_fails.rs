use svm_sdk_mock::template;

#[template]
mod Template {
    #[ctor]
    #[endpoint]
    fn get() {}
}

fn main() {}
