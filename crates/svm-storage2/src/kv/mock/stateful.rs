use std::{cell::RefCell, collections::HashMap, rc::Rc};

use svm_common::State;
use svm_kv::traits::KVStore;

struct Node {
    state: State,

    data: HashMap<Vec<u8>, Vec<u8>>,

    prev: Option<Box<Node>>,
}

impl Node {
    fn empty() -> Self {
        Self {
            state: State::empty(),
            data: HashMap::new(),
            prev: None,
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key_vec = key.to_vec();

        match self.data.get(&key_vec) {
            Some(value) => Some(value.to_vec()),
            None => match &self.prev {
                None => None,
                Some(p) => p.get(key),
            },
        }
    }
}

pub struct StatefulKV {
    state: State,

    head: Option<Node>,
}

impl KVStore for StatefulKV {
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.head.as_ref().and_then(|h| h.get(key))
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        let data = changes
            .iter()
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
            .collect();

        let old_head = self.head.take();
        let state = State::empty();
        let mut head = Node::empty();

        head.data = data;
        head.state = new_state.clone();
        head.prev = old_head.map(|head| Box::new(head));

        self.state = state;
        self.head = Some(head);
    }
}

impl StatefulKV {
    pub fn new() -> Self {
        Self {
            state: State::empty(),
            head: None,
        }
    }
}
