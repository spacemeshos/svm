use svm_sdk::template;

#[template]
mod Template {
    #[ctor]
    #[endpoint]
    fn get() {}
}

fn main() {}
