#![allow(unused)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(windows))] {
        use svm_sdk::Amount;
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
        fn add(a: i32, b: i32) -> (Amount, bool) {
            let c = a + b;

            let amount = Amount(c as u64);
            let err = false;

            return (amount, err)
        }

        #[test]
        fn test_endpoint() {
            set_calldata((10i32, 20i32));

            add();

            let mut returndata = get_returndata();

            let (amount, err): (Amount, bool) = returndata.next_2();

            assert_eq!(amount, Amount(30));
            assert_eq!(err, false);
        }
    }
}
