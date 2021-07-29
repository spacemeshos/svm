use std::collections::HashMap;
use std::fmt::Debug;

impl DbBackend for rocksdb::DB {
    type Error = rocksdb::Error;

    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
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

    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(HashMap::get(self, key).cloned())
    }

    fn upsert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(HashMap::insert(self, key.to_vec(), value.to_vec()))
    }
}

pub trait DbBackend {
    type Error: Debug;

    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;

    fn upsert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;
}
