use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        flag: bool,

        flags: [bool; 3],
    }
}

fn main() {
    // `flag`
    let flag = Storage::get_flag();
    assert_eq!(flag, false);

    Storage::set_flag(true);
    let flag = Storage::get_flag();
    assert!(flag);

    // `flags`
    let flag0 = Storage::get_flags(0);
    let flag1 = Storage::get_flags(1);
    let flag2 = Storage::get_flags(2);
    assert_eq!(flag0, false);
    assert_eq!(flag1, false);
    assert_eq!(flag2, false);

    Storage::set_flags(0, true);
    Storage::set_flags(2, true);

    let flag0 = Storage::get_flags(0);
    let flag1 = Storage::get_flags(1);
    let flag2 = Storage::get_flags(2);
    assert_eq!(flag0, true);
    assert_eq!(flag1, false);
    assert_eq!(flag2, true);
}
