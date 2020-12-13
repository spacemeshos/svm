use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        word: i16,
    }
}

fn main() {
    let word = Storage::get_word();
    assert_eq!(word, 0);

    Storage::set_word(-(0x1020));
    let word = Storage::get_word();
    assert_eq!(word, -(0x1020));
}
