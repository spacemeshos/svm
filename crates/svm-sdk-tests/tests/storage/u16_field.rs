use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        word: u16,
    }
}

fn main() {
    let word = Storage::get_word();
    assert_eq!(word, 0);

    Storage::set_word(0xABCD);
    let word = Storage::get_word();
    assert_eq!(word, 0xABCD);
}
