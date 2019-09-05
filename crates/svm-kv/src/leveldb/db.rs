use crate::leveldb::LDBKey;
use crate::traits::KVStore;

use db_key::Key;

use std::path::Path;

use leveldb::database::{
    batch::{Batch, Writebatch},
    kv::KV,
    Database,
};
use leveldb::options::{Options, ReadOptions, WriteOptions};

/// An implementation of `KVStore` trait against `LevelDB`.
pub struct LDBStore {
    db: Database<LDBKey>,
}

impl LDBStore {
    pub fn new(path: &Path) -> Self {
        let mut opts = Options::new();
        opts.create_if_missing = true;

        let db = match Database::open(path, opts) {
            Ok(db) => db,
            Err(e) => panic!("failed to open database: {:?}", e),
        };

        Self { db }
    }
}

impl KVStore for LDBStore {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let ldb_key = LDBKey(key.to_vec());

        let read_opts = ReadOptions::new();
        let res = self.db.get(read_opts, ldb_key);
        match res {
            Ok(data) => data,
            Err(_) => panic!("failed reading data"),
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        let mut batch = Writebatch::<LDBKey>::new();

        for (k, v) in changes {
            let k = LDBKey::from_u8(k);

            batch.put(k, v);
        }

        let res = self.db.write(WriteOptions::new(), &batch);

        if res.is_err() {
            panic!("failed writing data");
        }
    }

    fn close(mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leveldb_sanity() {
        let mut db = LDBStore::new(Path::new("leveldb-tests"));

        db.store(&[(&[10, 20, 30], &[40, 50, 60])]);

        let v = db.get(&[10, 20, 30]).unwrap();
        assert_eq!(vec![40, 50, 60], v);

        db.close();

        let mut db = LDBStore::new(Path::new("leveldb-tests"));
        let v = db.get(&[10, 20, 30]).unwrap();
        assert_eq!(vec![40, 50, 60], v);
    }
}
