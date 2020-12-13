use svm_sdk::app;

#[app]
mod App {
    #[storage]
    struct Storage {
        amount: svm_sdk::Amount,

        amounts: [svm_sdk::Amount; 3],
    }
}

fn main() {
    // `amount`
    let amount = Storage::get_amount();
    assert_eq!(amount, svm_sdk::Amount(0));

    Storage::set_amount(svm_sdk::Amount(10));
    let amount = Storage::get_amount();
    assert_eq!(amount, svm_sdk::Amount(10));

    // `amounts`
    let amount0 = Storage::get_amounts(0);
    let amount1 = Storage::get_amounts(1);
    let amount2 = Storage::get_amounts(2);
    assert_eq!(amount0, svm_sdk::Amount(0));
    assert_eq!(amount1, svm_sdk::Amount(0));
    assert_eq!(amount2, svm_sdk::Amount(0));

    Storage::set_amounts(1, svm_sdk::Amount(10));

    let amount0 = Storage::get_amounts(0);
    let amount1 = Storage::get_amounts(1);
    let amount2 = Storage::get_amounts(2);
    assert_eq!(amount0, svm_sdk::Amount(0));
    assert_eq!(amount1, svm_sdk::Amount(10));
    assert_eq!(amount2, svm_sdk::Amount(0));
}
