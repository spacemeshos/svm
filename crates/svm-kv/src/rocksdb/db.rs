use crate::traits::KVStore;

use std::path::Path;

/// An implementation of `KVStore` trait against `rocksdb`.
pub struct RocksStore {
    closed: bool,
    pub(crate) db: rocksdb::DB,
}

impl RocksStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        println!("opening rocksdb. (path = \"{}\")", path.as_ref().display());

        Self {
            db: rocksdb::DB::open_default(path).unwrap(),
            closed: false,
        }
    }
}

impl KVStore for RocksStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        match self.db.get(key) {
            Ok(dbvec) => match dbvec {
                None => None,
                Some(dbvec) => Some(dbvec.to_vec()),
            },
            Err(_) => panic!("Error reading key: `{:?}`", key),
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        let mut batch = rocksdb::WriteBatch::default();

        for (k, v) in changes {
            batch.put(k, v);
        }

        let res = self.db.write(batch);

        if res.is_err() {
            panic!("failed writing data");
        }
    }

    fn close(&mut self) {
        if self.closed {
            return;
        }

        let path = self.db.path();

        println!("closing rocksdb. (path = \"{}\")", path.display());

        drop(&mut self.db);

        self.closed = true;
    }
}

impl Drop for RocksStore {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rocksdb_sanity() {
        let mut db = RocksStore::new("rocksdb-tests");

        db.store(&[(&[10, 20, 30], &[40, 50, 60])]);

        let v = db.get(&[10, 20, 30]).unwrap();
        assert_eq!(vec![40, 50, 60], v);

        drop(db);

        let mut db = RocksStore::new("rocksdb-tests");
        let v = db.get(&[10, 20, 30]).unwrap();
        assert_eq!(vec![40, 50, 60], v);
    }
}
