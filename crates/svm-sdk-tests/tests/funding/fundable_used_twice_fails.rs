use svm_sdk::app;

#[app]
mod App {
    #[fundable(allow)]
    #[fundable(allow)]
    #[endpoint]
    fn get(value: svm_sdk::Amount) {}
}

fn main() {}
