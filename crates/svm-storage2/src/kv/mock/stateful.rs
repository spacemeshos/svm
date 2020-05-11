use std::{cell::RefCell, collections::HashMap, rc::Rc};

use svm_common::{DefaultKeyHasher, KeyHasher, State};
use svm_kv::traits::KVStore;

#[derive(Debug)]
struct Node {
    state: State,

    data: HashMap<Vec<u8>, Vec<u8>>,

    prev: State,
}

impl Node {
    fn empty() -> Self {
        Self {
            state: State::empty(),
            data: HashMap::new(),
            prev: State::empty(),
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.get(key).map(|v| v.to_vec())
    }
}

pub struct StatefulKV {
    head: State,

    refs: HashMap<State, Box<Node>>,
}

impl KVStore for StatefulKV {
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let zeros = State::empty();

        let mut state = &self.head;

        loop {
            if state.as_slice() == zeros.as_slice() {
                return None;
            }

            let node = self.refs.get(&state).unwrap();

            match node.get(key) {
                None => state = &node.prev,
                Some(v) => return Some(v),
            }
        }

        unreachable!()
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        let changes: HashMap<_, _> = changes
            .iter()
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
            .collect();

        let old_state = &self.head;
        let new_state = self.compute_state(&changes, old_state);

        let mut node = Node::empty();
        node.state = new_state.clone();
        node.data = changes;
        node.prev = old_state.clone();

        self.head = new_state.clone();
        self.refs.insert(new_state, Box::new(node));
    }
}

impl StatefulKV {
    pub fn new() -> Self {
        Self {
            head: State::empty(),
            refs: HashMap::new(),
        }
    }

    pub fn rewind(&mut self, state: &State) {
        self.head = state.clone();
    }

    fn compute_state(&self, changes: &HashMap<Vec<u8>, Vec<u8>>, old_state: &State) -> State {
        let capacity = changes
            .iter()
            .fold(State::len(), |acc, (k, v)| acc + k.len() + v.len());

        let mut buf = Vec::with_capacity(capacity);

        buf.extend_from_slice(old_state.as_slice());

        for (k, v) in changes.iter() {
            buf.extend_from_slice(k);
            buf.extend_from_slice(v);
        }

        let bytes = DefaultKeyHasher::hash(&buf);
        debug_assert_eq!(bytes.len(), State::len());

        State::from(&bytes[..])
    }

    fn head_node(&self) -> Option<&Box<Node>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_kv_empty() {
        let kv = StatefulKV::new();
        assert_eq!(kv.head, State::empty());
    }

    #[test]
    fn mock_kv_single_state() {
        let mut kv = StatefulKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);

        let changes = vec![(&k1[..], &v1[..]), (&k2[..], &v2[..])];
        kv.store(&changes[..]);

        assert_ne!(kv.head, State::empty());

        assert_eq!(kv.get(&k1[..]).unwrap(), v1);
        assert_eq!(kv.get(&k2[..]).unwrap(), v2);
    }

    #[test]
    fn mock_kv_two_states() {
        let mut kv = StatefulKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);

        let changes = vec![(&k1[..], &v1[..]), (&k2[..], &v2[..])];
        kv.store(&changes[..]);

        let (k3, v3) = (b"ccc", vec![0x60, 0x70]);
        let (k4, v4) = (b"ddd", vec![0x80, 0x90]);

        let changes = vec![(&k3[..], &v3[..]), (&k4[..], &v4[..])];
        kv.store(&changes[..]);

        assert_eq!(kv.get(&k1[..]).unwrap(), v1);
        assert_eq!(kv.get(&k2[..]).unwrap(), v2);
        assert_eq!(kv.get(&k3[..]).unwrap(), v3);
        assert_eq!(kv.get(&k4[..]).unwrap(), v4);
    }

    #[test]
    fn mock_kv_three_states() {
        let mut kv = StatefulKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let changes = vec![(&k1[..], &v1[..])];
        kv.store(&changes[..]);

        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);
        let changes = vec![(&k2[..], &v2[..])];
        kv.store(&changes[..]);

        let (k3, v3) = (b"ccc", vec![0x60, 0x70]);
        let changes = vec![(&k3[..], &v3[..])];
        kv.store(&changes[..]);

        assert_eq!(kv.get(&k1[..]).unwrap(), v1);
        assert_eq!(kv.get(&k2[..]).unwrap(), v2);
        assert_eq!(kv.get(&k3[..]).unwrap(), v3);
    }

    #[test]
    fn mock_kv_update_a_key_value() {
        let mut kv = StatefulKV::new();

        let (k, v1) = (b"aaa", vec![0x10, 0x20]);

        let changes = vec![(&k[..], &v1[..])];
        kv.store(&changes[..]);

        assert_eq!(kv.get(&k[..]).unwrap(), v1);

        let (k, v2) = (b"aaa", vec![0x30, 0x40]);
        let changes = vec![(&k[..], &v2[..])];
        kv.store(&changes[..]);

        assert_eq!(kv.get(&k[..]).unwrap(), v2);
    }
}
