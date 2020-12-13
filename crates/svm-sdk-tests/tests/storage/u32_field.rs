use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        dword: u32,
    }
}

fn main() {
    let dword = Storage::get_dword();
    assert_eq!(dword, 0);

    Storage::set_dword(0xAABBCCDD);
    let dword = Storage::get_dword();
    assert_eq!(dword, 0xAABBCCDD);
}
