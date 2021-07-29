use svm_sdk::template;

#[template]
mod Template {
    #[ctor]
    fn initialize() {}

    #[endpoint]
    fn fail() {
        panic!()
    }
}
