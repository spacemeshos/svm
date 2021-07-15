use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        dword: i32,
        dwords: [i32; 3],
    }
}

fn main() {
    // `dword`
    let dword = Storage::get_dword();
    assert_eq!(dword, 0);

    Storage::set_dword(-(0x00102030));
    let dword = Storage::get_dword();
    assert_eq!(dword, -(0x00102030));

    // `dwords`
    let dword0 = Storage::get_dwords(0);
    let dword1 = Storage::get_dwords(1);
    let dword2 = Storage::get_dwords(2);
    assert_eq!(dword0, 0);
    assert_eq!(dword1, 0);
    assert_eq!(dword2, 0);

    Storage::set_dwords(0, -(0x00102030));
    Storage::set_dwords(1, -(0x00405060));
    Storage::set_dwords(2, -(0x0708090));

    let dword0 = Storage::get_dwords(0);
    let dword1 = Storage::get_dwords(1);
    let dword2 = Storage::get_dwords(2);
    assert_eq!(dword0, -(0x00102030));
    assert_eq!(dword1, -(0x00405060));
    assert_eq!(dword2, -(0x00708090));
}
