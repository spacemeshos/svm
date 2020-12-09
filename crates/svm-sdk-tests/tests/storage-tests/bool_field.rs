use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        flag: bool,
    }
}

fn main() {
    // let v = Storage::get_flag();
    // assert!(v == false);
}
