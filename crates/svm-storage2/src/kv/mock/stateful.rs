use std::cell::RefCell;
use std::rc::Rc;

use crate::kv::KV;

use svm_common::State;

pub struct StatefulKV {
    state: State,

    raw_kv: Rc<RefCell<dyn KV>>,
}

impl KV for StatefulKV {
    fn get(&self, rel_key: &[u8]) -> Option<Vec<u8>> {
        let abs_key = self.build_key(rel_key);

        self.raw_kv.borrow().get(&abs_key)
    }

    fn set(&mut self, changes: &[(Vec<u8>, Vec<u8>)]) {
        todo!()
    }
}

impl StatefulKV {
    pub fn new(state: State, raw_kv: Rc<RefCell<dyn KV>>) -> Self {
        Self { state, raw_kv }
    }

    fn build_key(&self, rel_key: &[u8]) -> Vec<u8> {
        let cap = State::len() + rel_key.len();
        let buf: Vec<u8> = Vec::with_capacity(cap);

        todo!()
    }
}

pub fn commit(kv: Rc<RefCell<dyn KV>>) -> State {
    todo!()
}
