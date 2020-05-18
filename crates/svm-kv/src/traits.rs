/// `KVStore` is a trait for defining an interface against key-value stores, for example `rocksdb/leveldb`.
pub trait KVStore {
    /// Retrieves the value pointed by `key` (Optional).
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Stores a batch of changes. Each change is of `(ns, key) -> value` association.
    fn store(&mut self, changes: &[(&[u8], &[u8])]);
}
