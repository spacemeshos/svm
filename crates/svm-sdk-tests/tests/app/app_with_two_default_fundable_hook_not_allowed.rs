use svm_sdk::app;

#[app]
mod App {
    #[fundable_hook(default)]
    fn allow() {}

    #[fundable_hook(default)]
    fn deny() {
        panic!()
    }
}

fn main() {}
