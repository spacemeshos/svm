use super::LDBKey;
use crate::traits::KVStore;

use std::borrow::Borrow;
use std::path::{Path, PathBuf};

use db_key::Key;
use leveldb::database::{kv::KV, Database};
use leveldb::options::{Options, ReadOptions, WriteOptions};

pub struct LDBStore {
    db: Database<Vec<u8>>,
}

// impl<'ldb> LDBStore<'ldb> {
//     fn new(path: &Path) -> Self {
//         let mut opts = Options::new();
//         opts.create_if_missing = true;
//
//         let db = match Database::open(path, opts) {
//             Ok(db) => db,
//             Err(e) => panic!("failed to open database: {:?}", e),
//         };
//
//         Self { db }
//     }
// }
//
// impl KVStore for LDBStore {
//     fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
//         let ldb_key = LDBKey(key);
//
//         let read_opts = ReadOptions::new();
//         let res = self.db.get(read_opts, ldb_key);
//         match res {
//             Ok(data) => assert!(data.is_none()),
//             Err(_) => panic!("failed reading data"),
//         }
//         unimplemented!()
//     }
//
//     fn store(&mut self, changes: &[(&[u8], &[u8])]) {
//         //
//     }
// }
