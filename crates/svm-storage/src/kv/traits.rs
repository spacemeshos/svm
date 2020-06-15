use svm_common::State;

/// This trait should be implemented by `State`-aware key-value stores.
///
/// Implementations of this trait will probably want to use a raw-key value store
/// for implementating primitive operations.
///
pub trait StatefulKV {
    /// Gets the `value` pointed by by `key`.
    ///
    /// If there's a matching pending `value` (i.e not flushed yet) it should be returned.
    /// Otherwise, should proceed looking for `key -> value` under the persisted data.
    ///
    /// In case there is no matching `value`, `None` should be returned.
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Sets a new pending change.
    ///
    /// Calling `set` should not persist the change but mark it as a pending change.
    /// when calling the `flush` method.
    ///
    /// Subsequent `get` calls on `key` should return the new set `value`.
    ///
    /// See also: `flush`
    fn set(&mut self, key: &[u8], value: &[u8]);

    /// Discards all pending changes
    ///
    /// This functionality is useful when an executed application transaction has failed.
    /// When that happens, we want to discard its changes.
    ///
    /// All the pending changes since the last `checkpoint` will be discarded.
    ///
    /// Any other pending changes associated with a checkpoint won't be discarded,
    /// even if `flush` has not been called on them.
    ///
    fn discard(&mut self);

    /// Persists all pending changes since last `flush` call.
    ///
    /// A single `flush` call might invlove persisting a coupl of checkpoints.
    ///
    /// # Panics
    ///
    /// Traits implementations are expected to panic in cases there are pending changes
    /// not associated with any checkpoint.
    ///
    /// In order to aviod that, a call to `checkpoint` should be invoked prior to calling `flush`.
    ///
    fn flush(&mut self);

    /// Creates a new checkpoint and returns its `State`.
    ///
    /// In order to persist the pending changes, `flush` should be called.
    #[must_use]
    fn checkpoint(&mut self) -> State;

    /// Rewinds the current pointed-to `State`.
    ///
    /// # Panics
    ///
    /// Traits implementations are expected to panic in cases there are pending changes.
    ///
    /// See also: `checkpoint` and `flush`.
    ///
    fn rewind(&mut self, state: &State);

    /// Returns the current `State`.
    ///
    /// Its value should be the last created checkpoint `State`.
    /// That checkpoint may or may not be persisted. (depends whether `flush` has been called).
    ///
    /// In case no new checkpoints have been created yet,
    /// should return the `State` given during initialization.
    #[must_use]
    fn head(&self) -> State;
}
