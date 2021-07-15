use svm_sdk::{template, Address};

#[template]
mod Template {
    #[storage]
    struct Storage {
        addr: Address,
    }

    #[ctor]
    fn initialize() {
        //
    }

    #[endpoint]
    fn store_addr(addr: Address) {
        Storage::set_addr(&addr);
    }

    #[endpoint]
    fn load_addr() -> Address {
        // panic!()
        Storage::get_addr()
    }
}
