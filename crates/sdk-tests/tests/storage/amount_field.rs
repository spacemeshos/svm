use svm_sdk::{template, Amount};

#[template]
mod Template {
    #[storage]
    struct Storage {
        amount: Amount,
        amounts: [Amount; 3],
    }
}

fn main() {
    // `amount`
    let amount = Storage::get_amount();
    assert_eq!(amount, Amount(0));

    Storage::set_amount(Amount(10));
    let amount = Storage::get_amount();
    assert_eq!(amount, Amount(10));

    // `amounts`
    let amount0 = Storage::get_amounts(0);
    let amount1 = Storage::get_amounts(1);
    let amount2 = Storage::get_amounts(2);
    assert_eq!(amount0, Amount(0));
    assert_eq!(amount1, Amount(0));
    assert_eq!(amount2, Amount(0));

    Storage::set_amounts(1, Amount(10));

    let amount0 = Storage::get_amounts(0);
    let amount1 = Storage::get_amounts(1);
    let amount2 = Storage::get_amounts(2);
    assert_eq!(amount0, Amount(0));
    assert_eq!(amount1, Amount(10));
    assert_eq!(amount2, Amount(0));
}
