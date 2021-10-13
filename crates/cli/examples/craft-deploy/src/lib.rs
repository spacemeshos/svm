use svm_sdk::{template, Address};

#[template]
mod Template {
    #[storage]
    struct Storage {
        addr: Address,
    }

    #[ctor]
    fn initialize(init: Address, should_store: bool) -> bool {
        if should_store {
            Storage::set_addr(&init);
        }

        true
    }

    #[endpoint]
    fn store_addr(addr: Address) {
        Storage::set_addr(&addr);
    }

    #[endpoint]
    fn load_addr() -> Address {
        Storage::get_addr()
    }
}
