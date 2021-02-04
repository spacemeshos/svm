use svm_sdk::{app, ensure};
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
    fn withdraw(value: Amount, to: Address) {
        do_auth();

        let balance = Storage::get_balance();

        ensure!(balance >= value, "`value` is larger than vault's `balance`");

        let new_balance = balance - value;

        Storage::set_balance(new_balance);

        Node::transfer(&to, value);
    }

    #[fundable_hook]
    fn default(value: Amount) {
        let old = Storage::get_balance();
        let new = old + value;

        Storage::set_balance(new);
    }

    fn do_auth() {
        // TODO: have `Node::public_key()`

        let m = Address::from([0x10; Address::len()]);
        let master = Storage::get_master();

        ensure!(m == master, "wrong master key!");
    }
}
