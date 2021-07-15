use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        qword: u64,
        qwords: [u64; 3],
    }
}

fn main() {
    // `qword`
    let qword = Storage::get_qword();
    assert_eq!(qword, 0);

    Storage::set_qword(0xAAAAAAAA_AAAAAAAA);
    let qword = Storage::get_qword();
    assert_eq!(qword, 0xAAAAAAAA_AAAAAAAA);

    // `qwords`
    let qword0 = Storage::get_qwords(0);
    let qword1 = Storage::get_qwords(1);
    let qword2 = Storage::get_qwords(2);
    assert_eq!(qword0, 0);
    assert_eq!(qword1, 0);
    assert_eq!(qword2, 0);

    Storage::set_qwords(0, 0xAAAAAAAA_AAAAAAAA);
    Storage::set_qwords(1, 0xBBBBBBBB_BBBBBBBB);
    Storage::set_qwords(2, 0xCCCCCCCC_CCCCCCCC);

    let qword0 = Storage::get_qwords(0);
    let qword1 = Storage::get_qwords(1);
    let qword2 = Storage::get_qwords(2);
    assert_eq!(qword0, 0xAAAAAAAA_AAAAAAAA);
    assert_eq!(qword1, 0xBBBBBBBB_BBBBBBBB);
    assert_eq!(qword2, 0xCCCCCCCC_CCCCCCCC);
}
