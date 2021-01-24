use svm_sdk::app;

#[app]
mod App {
    #[fundable_hook]
    fn deny(v: svm_sdk::Amount) {}
}

fn main() {}
