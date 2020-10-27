#![allow(unused)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(windows))] {

        use svm_sdk::Amount;
        use svm_sdk::host::{MockHost, traits::Host};
        use svm_sdk_macros::endpoint;

        #[endpoint]
        fn add(a: i32, b: i32) -> (Amount, bool) {
            let c = a + b;

            return (Amount(c as u64), false)
        }

        #[test]
        fn test_endpoint() {
            use svm_abi_decoder::CallData;

            {
                let host = MockHost::instance();

                let calldata = (10i32, 20i32);
                host.set_calldata(calldata);
            }

            add();

            {
                use svm_abi_decoder::CallData;

                let host = MockHost::instance();

                let bytes = host.get_returndata();

                let bytes: &'static [u8] = bytes.unwrap().leak();
                let mut returndata = CallData::new(&bytes);

                let (amount, err): (Amount, bool) = returndata.next_2();

                assert_eq!(amount, Amount(30));
                assert_eq!(err, false);
            }
        }
    }
}
