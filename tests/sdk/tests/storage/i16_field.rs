use svm_sdk_mock::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        word: i16,
        words: [i16; 3],
    }
}

fn main() {
    // `word`
    let word = Storage::get_word();
    assert_eq!(word, 0);

    Storage::set_word(-(0x0010));
    let word = Storage::get_word();
    assert_eq!(word, -(0x0010));

    // `words`
    let word0 = Storage::get_words(0);
    let word1 = Storage::get_words(1);
    let word2 = Storage::get_words(2);
    assert_eq!(word0, 0);
    assert_eq!(word1, 0);
    assert_eq!(word2, 0);

    Storage::set_words(0, -(0x1020));
    Storage::set_words(1, -(0x3040));
    Storage::set_words(2, -(0x5060));

    let word0 = Storage::get_words(0);
    let word1 = Storage::get_words(1);
    let word2 = Storage::get_words(2);
    assert_eq!(word0, -(0x1020));
    assert_eq!(word1, -(0x3040));
    assert_eq!(word2, -(0x5060));
}
