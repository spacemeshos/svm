use svm_common::State;
use svm_kv::traits::KVStore;

/// This trait should be implemented by `State`-aware key-value stores.
pub trait StatefulKVStore: KVStore {
    /// Rewind to an historical `State`
    fn rewind(&mut self, state: &State);

    /// The current pointed-by `State`.
    #[must_use]
    fn head(&self) -> State;
}
