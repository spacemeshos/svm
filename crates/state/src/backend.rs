use std::collections::HashMap;
use std::fmt::Debug;

/// The underlying storage layer that handles the disk persistance of a
/// [`GlobalState`](crate::GlobalState).
pub trait DbBackend {
    /// The type for any error condition that may happen when fetching/saving
    /// data.
    type Error: Debug;

    /// Fetches a `key` from disk, which may or may be found.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;

    /// Either persists `key` associating it to a `value` or updates its value.
    /// If applicable, returns the previous value associated with `key`.
    fn upsert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;
}

impl DbBackend for rocksdb::DB {
    type Error = rocksdb::Error;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        rocksdb::DB::get(self, key)
    }

    fn upsert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        let old_value = self.get(key)?;
        rocksdb::DB::put(self, key, value)?;
        Ok(old_value)
    }
}

impl DbBackend for HashMap<Vec<u8>, Vec<u8>> {
    type Error = ();

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(HashMap::get(self, key).cloned())
    }

    fn upsert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(HashMap::insert(self, key.to_vec(), value.to_vec()))
    }
}
