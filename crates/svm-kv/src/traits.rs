/// A trait for defining an interface against a raw key-value store (e.g `leveldb/rocksdb`).
pub trait RawKV {
    /// # Gets the `value` pointed by by `key`.
    ///
    /// If there's a matchikg pending `value` (i.e not flushed yet) it should be returned.
    /// Otherwise, should proceed looking for `key -> value` under the persisted data.
    ///
    /// In case there is no matching `value`, `None` should be returned.
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Stores a batch of changes.
    ///
    /// Each change is tuple denoting `(key, value)`
    fn set(&mut self, changes: &[(&[u8], &[u8])]);
}
