use svm_sdk::template;

#[template]
mod Template {
    #[endpoint]
    #[fundable(deny)]
    fn init() {}
}

fn main() {}
