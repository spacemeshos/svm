use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        name: String,
    }
}

fn main() {}
