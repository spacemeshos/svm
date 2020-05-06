use std::cell::RefCell;
use std::rc::Rc;

use svm_common::State;
use svm_kv::traits::KVStore;

pub struct StatefulKV {
    state: State,

    raw_kv: Rc<RefCell<dyn KVStore>>,
}

impl KVStore for StatefulKV {
    #[must_use]
    fn get(&self, rel_key: &[u8]) -> Option<Vec<u8>> {
        let abs_key = self.build_key(rel_key);

        // self.raw_kv.borrow().get(&abs_key)
        todo!()
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        todo!()
    }
}

impl StatefulKV {
    pub fn new(state: State, raw_kv: Rc<RefCell<dyn KVStore>>) -> Self {
        Self { state, raw_kv }
    }

    fn build_key(&self, rel_key: &[u8]) -> Vec<u8> {
        let cap = State::len() + rel_key.len();
        let buf: Vec<u8> = Vec::with_capacity(cap);

        todo!()
    }
}

pub fn commit(kv: Rc<RefCell<dyn KVStore>>) -> State {
    todo!()
}
