use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage1 {
        addr: Address,
    }

    #[storage]
    struct Storage2 {
        amount: Amount,
    }
}

fn main() {}
