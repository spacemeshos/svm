use svm_sdk::{app, Address};

use svm_sdk_tests::{call_1, call_2};

#[app]
mod App {
    #[endpoint]
    fn zeros() -> Address {
        [0; Address::len()].into()
    }

    #[endpoint]
    fn identity(a: Address) -> Address {
        a
    }

    #[endpoint]
    fn swap(a: Address, b: Address) -> (Address, Address) {
        (b, a)
    }
}

fn test_zeros() {
    let params = Vec::<Address>::new();
    let res: Address = call_1(zeros, params);

    let zeros_addr: Address = [0; Address::len()].into();
    assert_eq!(res, zeros_addr);
}

fn test_identity() {
    let addr: Address = [0x10; Address::len()].into();

    let res: Address = call_1(identity, vec![addr.clone()]);
    assert_eq!(res, addr);
}

fn test_swap() {
    let a: Address = [0x10; Address::len()].into();
    let b: Address = [0x20; Address::len()].into();

    let res: (Address, Address) = call_2(swap, vec![a.clone(), b.clone()]);
    assert_eq!(res, (b, a));
}

fn main() {
    test_zeros();
    test_identity();
    test_swap();
}
