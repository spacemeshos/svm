use svm_sdk::template;

#[template]
mod Template {
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
