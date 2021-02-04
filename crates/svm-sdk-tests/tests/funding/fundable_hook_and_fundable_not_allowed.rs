use svm_sdk::app;

#[app]
mod App {
    #[fundable_hook]
    #[fundable(default)]
    fn get() {}
}

fn main() {}
