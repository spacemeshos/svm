use svm_sdk::{app, Address};

#[app]
mod App {
    #[storage]
    struct Storage {
        addr: Address,
    }
}

fn main() {
    let addr = Storage::get_addr();
    assert_eq!(addr, Address::from([0; Address::len()]));

    let addr1 = Address::from([0x10; Address::len()]);
    Storage::set_addr(&addr1);

    let addr = Storage::get_addr();
    assert_eq!(addr, addr1);
}
