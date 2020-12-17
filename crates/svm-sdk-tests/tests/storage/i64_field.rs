use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        qword: i64,

        qwords: [i64; 3],
    }
}

fn main() {
    // `qword`
    let qword = Storage::get_qword();
    assert_eq!(qword, 0);

    Storage::set_qword(-(0x00AAAAAA_AAAAAAAA));
    let qword = Storage::get_qword();
    assert_eq!(qword, -(0x00AAAAAA_AAAAAAAA));

    // `qwords`
    let qword0 = Storage::get_qwords(0);
    let qword1 = Storage::get_qwords(1);
    let qword2 = Storage::get_qwords(2);
    assert_eq!(qword0, 0);
    assert_eq!(qword1, 0);
    assert_eq!(qword2, 0);

    Storage::set_qwords(0, -(0x00AAAAAA_AAAAAAAA));
    Storage::set_qwords(1, -(0x00BBBBBB_BBBBBBBB));
    Storage::set_qwords(2, -(0x00CCCCCC_CCCCCCCC));

    let qword0 = Storage::get_qwords(0);
    let qword1 = Storage::get_qwords(1);
    let qword2 = Storage::get_qwords(2);
    assert_eq!(qword0, -(0x00AAAAAA_AAAAAAAA));
    assert_eq!(qword1, -(0x00BBBBBB_BBBBBBBB));
    assert_eq!(qword2, -(0x00CCCCCC_CCCCCCCC));
}
