use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        byte: i8,
        bytes: [i8; 3],
    }
}

fn main() {
    // `byte`
    let byte = Storage::get_byte();
    assert_eq!(byte, 0);

    Storage::set_byte(-10);
    let byte = Storage::get_byte();
    assert_eq!(byte, -10);

    // `bytes`
    let byte0 = Storage::get_bytes(0);
    let byte1 = Storage::get_bytes(1);
    let byte2 = Storage::get_bytes(2);
    assert_eq!(byte0, 0);
    assert_eq!(byte1, 0);
    assert_eq!(byte2, 0);

    Storage::set_bytes(0, -10);
    Storage::set_bytes(1, -20);
    Storage::set_bytes(2, -30);

    let byte0 = Storage::get_bytes(0);
    let byte1 = Storage::get_bytes(1);
    let byte2 = Storage::get_bytes(2);
    assert_eq!(byte0, -10);
    assert_eq!(byte1, -20);
    assert_eq!(byte2, -30);
}
