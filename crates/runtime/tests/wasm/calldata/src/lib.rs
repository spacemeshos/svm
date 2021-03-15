use svm_sdk::{app, Address};

#[app]
mod App {
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
        Storage::get_addr()
    }
}
