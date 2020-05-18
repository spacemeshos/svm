#[macro_export]
macro_rules! assert_key_value {
    ($kv:expr, $key:expr, $expected:expr) => {{
        let actual = $kv.get(&$key).unwrap();
        assert_eq!($expected, &actual[..]);
    }};
}

#[macro_export]
macro_rules! assert_no_key {
    ($kv:expr, $key:expr) => {{
        assert!($kv.get(&$key).is_none());
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
    ($expected:expr, $actual:expr) => {{
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
