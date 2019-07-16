use std::path::Path;

use crate::level_key::LevelKey32;
use crate::traits::KVStore;

use leveldb::batch::{Batch, Writebatch};
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};

/// An interface against the `LevelDB` database.
///
/// * Working with 32 bytes key-size.
/// * Implements the `KVStore` trait.
pub struct LevelDB {
    db: Database<LevelKey32>,
}

impl LevelDB {
    /// Creates a new instance of a `LevelDB` client
    pub fn new(path: &Path, opts: Options) -> Self {
        let db = Database::open(path, opts).unwrap();

        Self { db }
    }
}

impl KVStore for LevelDB {
    type K = LevelKey32;

    #[must_use]
    fn get(&self, key: Self::K) -> Option<Vec<u8>> {
        match self.db.get(ReadOptions::new(), key) {
            Ok(None) => None,
            Ok(v) => v,
            Err(_) => panic!(format!("Couldn't read leveldb key {:?}", key)),
        }
    }

    fn store(&mut self, changes: &[(Self::K, &[u8])]) {
        let mut batch: Writebatch<Self::K> = Writebatch::new();

        for (k, v) in changes.iter() {
            batch.put(*k, v);
        }

        let res = self.db.write(WriteOptions::new(), &batch);

        match res {
            Ok(_) => (),
            Err(_) => panic!("Couldn't store changes into leveldb"),
        }
    }
}
