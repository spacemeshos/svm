use svm_sdk::app;

#[app]
mod App {
    #[ctor]
    #[fundable(deny)]
    fn init() {}
}

fn main() {}
