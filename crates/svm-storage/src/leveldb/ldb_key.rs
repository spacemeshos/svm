use std::borrow::Borrow;

use db_key::Key;

/// Implements `db_key::Key`
pub struct LDBKey(pub Vec<u8>);

impl Key for LDBKey {
    fn from_u8(key: &[u8]) -> Self {
        Self(key.to_vec())
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        f(self.0.as_slice())
    }
}

impl Borrow<[u8]> for LDBKey {
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl std::fmt::Debug for LDBKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ldb_key_borrow() {
        let expected = LDBKey(vec![10, 20, 30]);

        let actual = LDBKey::from_u8(&[10, 20, 30]);

        assert_eq!(expected.0, actual.0);
    }
}
