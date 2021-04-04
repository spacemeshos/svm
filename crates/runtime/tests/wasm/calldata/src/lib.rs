use svm_sdk::{app, Address, Amount};

#[app]
mod App {
    // #[storage]
    // struct Storage {
    //     // addr: Address,
    //     amount: Amount,
    // }

    #[ctor]
    fn initialize() {
        //
    }

    #[endpoint]
    fn f(v: bool) {}

    // #[endpoint]
    // fn load_amount() -> Amount {
    //     Storage::get_amount()
    // }

    // #[endpoint]
    // fn store_addr(addr: Address) {
    //     Storage::set_addr(&addr);
    // }

    // #[endpoint]
    // fn load_addr() -> Address {
    //     // panic!()
    //     Storage::get_addr()
    // }
}
