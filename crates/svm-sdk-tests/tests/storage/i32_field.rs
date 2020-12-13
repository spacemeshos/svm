use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        dword: i32,
    }
}

fn main() {
    let dword = Storage::get_dword();
    assert_eq!(dword, 0);

    Storage::set_dword(-(0x10203040));
    let dword = Storage::get_dword();
    assert_eq!(dword, -(0x10203040));
}
