use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        word: u16,
        words: [u16; 3],
    }
}

fn main() {
    // `word`
    let word = Storage::get_word();
    assert_eq!(word, 0);

    Storage::set_word(0xABCD);
    let word = Storage::get_word();
    assert_eq!(word, 0xABCD);

    // `words`
    let word0 = Storage::get_words(0);
    let word1 = Storage::get_words(1);
    let word2 = Storage::get_words(2);
    assert_eq!(word0, 0);
    assert_eq!(word1, 0);
    assert_eq!(word2, 0);

    Storage::set_words(0, 0xAAAA);
    Storage::set_words(1, 0xBBBB);
    Storage::set_words(2, 0xCCCC);

    let word0 = Storage::get_words(0);
    let word1 = Storage::get_words(1);
    let word2 = Storage::get_words(2);
    assert_eq!(word0, 0xAAAA);
    assert_eq!(word1, 0xBBBB);
    assert_eq!(word2, 0xCCCC);
}
