use svm_sdk::app;
use svm_sdk::{Address, Amount};

#[app]
mod App {
    #[storage]
    struct Storage {
        master: Address,

        balance: Amount,
    }

    // TODO:
    // ====
    //
    // implement in `svm-sdk` the following:
    //
    // 1. add `[ctor]`
    // 2. add `[fixed_gas = 100]`
    // 3. add `[doc("...")]`
    // 4. default `fundable hook`

    #[fundable(default)]
    #[endpoint]
    fn init(master: Address) {
        Storage::set_master(&master);
    }

    #[fundable(default)]
    #[endpoint]
    fn fund() {
        //
    }

    #[fundable_hook]
    fn default(value: Amount) {
        let old = Storage::get_balance();
        let new = old + value;

        Storage::set_balance(new);
    }
}
