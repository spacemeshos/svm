// #![allow(unused)]

// use cfg_if::cfg_if;

// cfg_if! {
//     if #[cfg(not(windows))] {
//         use svm_sdk::{Amount, Address, CallData};
//         use svm_sdk::host::MockHost;
//         use svm_sdk::traits::{Encoder, Host};

//         use svm_sdk_macros::endpoint;

//         use std::sync::Mutex;

//         use lazy_static::lazy_static;

//         lazy_static! {
//             static ref TEST_LOCK: Mutex<()> = Mutex::new(());
//         }

//         fn set_calldata<T: Encoder>(calldata: T)  {
//             let host = MockHost::instance();

//             host.set_calldata(calldata);
//         }

//         fn get_returndata() -> CallData {
//             let host = MockHost::instance();
//             let bytes = host.get_returndata();

//             let bytes = bytes.unwrap().leak();
//             let mut returndata = CallData::new(bytes);

//             returndata
//         }

//         fn test<F>(f: F) where F: FnOnce() {
//             // run the tests in a serial manner
//             let guard = TEST_LOCK.lock().unwrap();

//             let host = MockHost::instance();
//             host.reset();

//             f();
//         }

//         #[endpoint]
//         fn add(a: u8, b: u64, c: Amount) -> (Amount, bool) {
//             let a = Amount(a as u64);
//             let b = Amount(b as u64);

//             let amount = a + b + c;
//             let err = false;

//             return (amount, err)
//         }

//         #[endpoint]
//         fn first_or_second(first: bool, addr1: Address, addr2: Address) -> Address {
//             if first {
//                 addr1
//             }
//             else {
//                 addr2
//             }
//         }

//         #[test]
//         fn test_add() {
//             test(|| {
//                 let a = 10u8;
//                 let b = 20u64;
//                 let c = Amount(5);

//                 let calldata = (a, b, c);
//                 set_calldata(&calldata);

//                 add();

//                 let mut returndata = get_returndata();

//                 let (amount, err): (Amount, bool) = returndata.next_2();

//                 assert_eq!(amount, Amount(35));
//                 assert_eq!(err, false);
//             });
//         }

//         #[test]
//         fn test_first_or_second() {
//             test(|| {
//                 let addr2: Address = [0x10; Address::len()].into();
//                 let addr1: Address = [0x20; Address::len()].into();

//                 // 1) use `first = true`
//                 let calldata = (true, addr1, addr2);
//                 set_calldata(&calldata);

//                 first_or_second();

//                 let mut returndata = get_returndata();
//                 let addr: Address = returndata.next_1();
//                 assert_eq!(addr, addr1);

//                 // 2) use `first = false`
//                 let calldata = (false, addr1, addr2);
//                 set_calldata(&calldata);

//                 first_or_second();

//                 let mut returndata = get_returndata();
//                 let addr: Address = returndata.next_1();
//                 assert_eq!(addr, addr2);
//             });
//         }
//     }
// }
