/// `KVStore` is a trait for defining an interface against key-value store.
pub trait KVStore {
    /// Gets the `value` pointed by by `key`
    /// In case there is no matching `value` - `None` should be returned.
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Sets a new change to be stored upon `commit`.
    /// Calling `set` should not persist the change but save it for later
    /// when calling the `commit` method.
    ///
    /// See also: `commit`
    fn set(&mut self, key: &[u8], value: &[u8]);

    /// Commits all pending changes
    fn commit(&mut self);
}
