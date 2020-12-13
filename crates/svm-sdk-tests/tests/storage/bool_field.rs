use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        flag: bool,
    }
}

fn main() {
    let flag = Storage::get_flag();
    assert!(flag == false);

    Storage::set_flag(true);
    let flag = Storage::get_flag();
    assert!(flag);
}
