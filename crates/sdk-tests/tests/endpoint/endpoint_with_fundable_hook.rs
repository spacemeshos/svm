use svm_sdk::app;

#[app]
mod App {
    #[fundable_hook]
    #[endpoint]
    fn do_nothing() -> u8 {
        0
    }
}

fn main() {}
