use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        qword: i64,
    }
}

fn main() {
    let qword = Storage::get_qword();
    assert_eq!(qword, 0);

    Storage::set_qword(-(0x00AABBCC_DDEEFF00));
    let qword = Storage::get_qword();
    assert_eq!(qword, -(0x00AABBCC_DDEEFF00));
}
