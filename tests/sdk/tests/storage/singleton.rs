use svm_sdk_mock::template;

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
