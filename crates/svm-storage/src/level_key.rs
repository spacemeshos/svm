macro_rules! impl_leveldb_key {
    ($key_name: ident, $bytes_count: expr) => {
        /// Implements `db_key::Key` trait. The inner data is an array of `$bytes_count` bytes
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $key_name([u8; $bytes_count]);

        impl AsRef<[u8]> for $key_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl db_key::Key for $key_name {
            fn from_u8(key: &[u8]) -> Self {
                assert!(key.len() == $bytes_count);

                let mut bytes = [0; $bytes_count];
                bytes.copy_from_slice(key);

                Self(bytes)
            }

            fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
                f(&self.0)
            }
        }
    };
}

impl_leveldb_key!(LevelKey32, 32);

#[cfg(test)]
mod tests {
    use super::*;
    use db_key::Key;

    impl_leveldb_key!(LevelKeyTest3, 3);

    #[test]
    fn from_u8() {
        let key: LevelKeyTest3 = LevelKeyTest3::from_u8(&[10, 20, 30]);

        assert_eq!([10, 20, 30], key.0);
    }

    #[test]
    fn as_ref() {
        let key: LevelKeyTest3 = LevelKeyTest3::from_u8(&[10, 20, 30]);

        assert_eq!(&[10, 20, 30], key.as_ref());
    }

    #[test]
    fn as_slice() {
        let key: LevelKeyTest3 = LevelKeyTest3::from_u8(&[10, 20, 30]);
        let data: Vec<u8> = key.as_slice(|inner: &[u8]| inner.to_vec());

        assert_eq!(vec![10, 20, 30], data);
    }
}
