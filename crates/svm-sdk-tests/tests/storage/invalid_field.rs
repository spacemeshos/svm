use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        name: String,
    }
}

fn main() {}
