/// `KVStore` is a trait for defining an interface against key-value stores. for example `in-memory / rocksdb`
pub trait KVStore {
    /// Retrieves the value pointed by `key` (Optional).
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Stores a batch of changes. Each change is `key` -> `value` association.
    fn store(&mut self, changes: &[(&[u8], &[u8])]);
}
