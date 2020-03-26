#[macro_export]
macro_rules! assert_no_key {
    ($kv:expr, $ns:expr, $key: expr) => {{
        use svm_kv::traits::KVStore;

        assert!($kv.borrow().get(&$ns, &$key).is_none());
    }};
}

#[macro_export]
macro_rules! assert_key_value {
    ($kv:expr, $ns:expr, $key:expr, $expected:expr) => {{
        use svm_kv::traits::KVStore;

        let actual = $kv.borrow().get(&$ns, &$key).unwrap();
        assert_eq!($expected, &actual[..]);
    }};
}

#[macro_export]
macro_rules! assert_page_content {
    ($pages:ident, $page_idx:expr, $expected:expr) => {{
        assert_eq!($expected, $pages.read_page(PageIndex($page_idx)));
    }};
}

#[macro_export]
macro_rules! kv_keys_vec {
    ($kv:ident) => {{
        let keys: Vec<Vec<u8>> = $kv.borrow().keys().map(|key| key.clone()).collect();
        keys
    }};
}

#[macro_export]
macro_rules! assert_same_keys {
    ($expected: expr, $actual: expr) => {{
        let mut expected = $expected
            .iter()
            .map(|k| k.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let mut actual = $actual.to_vec();

        expected.sort();
        actual.sort();

        assert_eq!(&expected[..], &actual[..]);
    }};
}
