use super::LDBKey;
use crate::traits::KVStore;

use std::path::Path;

use leveldb::database::{kv::KV, Database};
use leveldb::options::{Options, ReadOptions};

/// An implementation of `KVStore` trait against `LevelDB`.
pub struct LDBStore {
    db: Database<LDBKey>,
}

impl LDBStore {
    fn new(path: &Path) -> Self {
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

    fn store(&mut self, _changes: &[(&[u8], &[u8])]) {
        unimplemented!()
    }
}
