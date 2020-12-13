use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        byte: i8,
    }
}

fn main() {
    let byte = Storage::get_byte();
    assert_eq!(byte, 0);

    Storage::set_byte(-10);
    let byte = Storage::get_byte();
    assert_eq!(byte, -10);
}
