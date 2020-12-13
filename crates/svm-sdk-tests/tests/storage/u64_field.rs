use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        qword: u64,
    }
}

fn main() {
    let qword = Storage::get_qword();
    assert_eq!(qword, 0);

    Storage::set_qword(0xAABBCCDD_EEFF0010);
    let qword = Storage::get_qword();
    assert_eq!(qword, 0xAABBCCDD_EEFF0010);
}
