use std::path::Path;

use crate::{key::concat_ns_to_key, traits::KVStore};

use log::info;

/// An implementation of `KVStore` trait against `rocksdb`.
pub struct Rocksdb {
    pub(crate) db: rocksdb::DB,
}

impl Rocksdb {
    /// New `Rocksdb` under the given `path`
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        info!("Opening rocksdb. (path = \"{}\")", path.as_ref().display());

        Self {
            db: rocksdb::DB::open_default(path).unwrap(),
        }
    }
}

impl KVStore for Rocksdb {
    #[allow(clippy::match_wild_err_arm)]
    fn get(&self, ns: &[u8], key: &[u8]) -> Option<Vec<u8>> {
        let key = concat_ns_to_key(ns, key);

        match self.db.get(&key) {
            Ok(dbvec) => match dbvec {
                None => None,
                Some(dbvec) => Some(dbvec.to_vec()),
            },
            Err(_) => panic!("Error reading key: `{:?}`", key),
        }
    }

    fn store(&mut self, ns: &[u8], changes: &[(&[u8], &[u8])]) {
        let mut batch = rocksdb::WriteBatch::default();

        for (k, v) in changes {
            let k = concat_ns_to_key(ns, k);

            let res = batch.put(k, v);

            if res.is_err() {
                panic!("failed `put`-ing bach data");
            }
        }

        let res = self.db.write(batch);

        if res.is_err() {
            panic!("failed storing changes.");
        }
    }
}

impl Drop for Rocksdb {
    fn drop(&mut self) {
        info!("dropping `Rocksdb`");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rocksdb_sanity() {
        let mut db = Rocksdb::new("rocksdb-tests");

        let ns = vec![0xFF, 0xFF];

        db.store(&ns, &[(&[10, 20, 30], &[40, 50, 60])]);

        let v = db.get(&ns, &[10, 20, 30]).unwrap();
        assert_eq!(vec![40, 50, 60], v);

        drop(db);

        let db = Rocksdb::new("rocksdb-tests");
        let v = db.get(&ns, &[10, 20, 30]).unwrap();
        assert_eq!(vec![40, 50, 60], v);
    }
}
