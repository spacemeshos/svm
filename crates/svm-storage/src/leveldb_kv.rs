use crate::traits::KVStore;

/// An interface for working against the `LevelDB` database
pub struct LevelDB;

impl KVStore for LevelDB {
    type K = [u8; 32];

    fn get(&self, key: Self::K) -> Option<Vec<u8>> {
        None
    }

    fn store(&mut self, changes: &[(Self::K, &[u8])]) {
        //
    }
}
