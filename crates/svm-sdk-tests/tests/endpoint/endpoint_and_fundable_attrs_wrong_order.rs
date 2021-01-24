use svm_sdk::app;

#[app]
mod App {
    #[endpoint]
    #[fundable(deny)]
    fn init() {}
}

fn main() {}
