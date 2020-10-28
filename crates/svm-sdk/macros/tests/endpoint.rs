#![allow(unused)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(windows))] {
        use svm_sdk::{Amount, Address};
        use svm_sdk::host::{MockHost, traits::Host};
        use svm_abi_encoder::Encoder;
        use svm_abi_decoder::CallData;
        use svm_sdk_macros::endpoint;

        fn set_calldata<T: Encoder>(calldata: T)  {
            let host = MockHost::instance();

            host.set_calldata(calldata);
        }

        fn get_returndata() -> CallData {
            let host = MockHost::instance();
            let bytes = host.get_returndata();

            let bytes = bytes.unwrap().leak();
            let mut returndata = CallData::new(bytes);

            returndata
        }

        #[endpoint]
        fn add(a: u8, b: u64, c: Amount, addr: Address) -> (Amount, bool) {
            let a = Amount(a as u64);
            let b = Amount(b as u64);

            let amount = a + b + c;
            let err = false;

            return (amount, err)
        }

        #[test]
        fn test_add() {
            let a = 10u8;
            let b = 20u64;
            let c = Amount(5);
            let addr: Address = [0x10; Address::len()].into();

            let calldata = (a, b, c, addr);
            set_calldata(calldata);

            add();

            let mut returndata = get_returndata();

            let (amount, err): (Amount, bool) = returndata.next_2();

            assert_eq!(amount, Amount(35));
            assert_eq!(err, false);
        }
    }
}
