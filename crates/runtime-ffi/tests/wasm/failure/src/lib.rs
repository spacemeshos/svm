use svm_sdk::{panic, template};

#[template]
mod Template {
    #[ctor]
    fn initialize() {}

    #[endpoint]
    fn fail() {
        panic();
    }
}
