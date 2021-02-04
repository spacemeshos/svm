use svm_sdk::app;

#[app]
mod App {
    #[fundable_hook]
    fn deny() -> u32 {
        0
    }
}

fn main() {}
