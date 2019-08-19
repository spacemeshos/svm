use db_key::Key;
use std::borrow::Borrow;

pub struct LDBKey<'a>(pub &'a [u8]);

// impl<'a> db_key::Key<'a> for LDBKey<'a> {}
//
// impl<'a> From<&'a [u8]> for LDBKey<'a> {
//     fn from(key: &'a [u8]) -> Self {
//         Self(key)
//     }
// }
//
// impl<'a> AsRef<[u8]> for LDBKey<'a> {
//     fn as_ref(&self) -> &[u8] {
//         self.0.as_ref()
//     }
// }

impl<'a> Borrow<[u8]> for LDBKey<'a> {
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn ldb_key_borrow() {
    //     let key = vec![10, 20, 30];
    //
    //     let ldb_key = LDBKey(key);
    // }
}
