use svm_sdk::{app, Amount};

#[app]
mod App {
    #[storage]
    struct Storage {
        amount: Amount,
    }
}

fn main() {
    let amount = Storage::get_amount();
    assert_eq!(amount, Amount(0));

    Storage::set_amount(Amount(10));
    let amount = Storage::get_amount();
    assert_eq!(amount, Amount(10));
}
