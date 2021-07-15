use svm_sdk::template;

#[template]
mod Template {
    #[storage]
    struct Storage {
        addr: svm_sdk::Address,
        addrs: [svm_sdk::Address; 3],
    }
}

fn main() {
    use svm_sdk::Address;

    let zero_addr = Address::from([0; Address::len()]);
    let ones_addr = Address::from([0xFF; Address::len()]);

    // `addr`
    let addr = Storage::get_addr();
    assert_eq!(addr, zero_addr);

    Storage::set_addr(&ones_addr);

    let addr = Storage::get_addr();
    assert_eq!(addr, ones_addr);

    // `addrs`
    let addr0 = Storage::get_addrs(0);
    let addr1 = Storage::get_addrs(1);
    let addr2 = Storage::get_addrs(2);
    assert_eq!(addr0, zero_addr);
    assert_eq!(addr1, zero_addr);
    assert_eq!(addr2, zero_addr);

    Storage::set_addrs(1, &ones_addr);

    let addr0 = Storage::get_addrs(0);
    let addr1 = Storage::get_addrs(1);
    let addr2 = Storage::get_addrs(2);
    assert_eq!(addr0, zero_addr);
    assert_eq!(addr1, ones_addr);
    assert_eq!(addr2, zero_addr);
}
