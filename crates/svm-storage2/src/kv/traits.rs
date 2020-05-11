use svm_common::State;
use svm_kv::traits::KVStore;

pub trait StatefulKVStore: KVStore {
    fn rewind(&mut self, state: &State);

    #[must_use]
    fn head(&self) -> State;
}
