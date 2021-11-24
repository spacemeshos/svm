use svm_sdk_mock::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        name: String,
    }
}

fn main() {}
