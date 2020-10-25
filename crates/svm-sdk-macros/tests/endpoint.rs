use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(windows))] {

        use svm_sdk_macros::endpoint;

        #[endpoint]
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        #[test]
        fn test_endpoint() {
            // let c = add(10, 20);

            // assert_eq!(c, 30);
        }
    }
}
