use svm_sdk::app;

#[app]
mod App {
    #[ctor]
    #[endpoint]
    fn get() {}
}

fn main() {}
