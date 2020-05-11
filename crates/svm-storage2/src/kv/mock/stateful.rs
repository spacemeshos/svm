use std::{cell::RefCell, collections::HashMap, rc::Rc};

use svm_common::{DefaultKeyHasher, KeyHasher, State};
use svm_kv::traits::KVStore;

#[derive(Debug)]
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
                Some(prev) => prev.get(key),
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
        let changes: HashMap<_, _> = changes
            .iter()
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
            .collect();

        let old_head = self.head.take();
        let mut head = Node::empty();

        head.state = self.compute_state(&changes, &self.state);
        head.data = changes;
        head.prev = old_head.map(|head| Box::new(head));

        self.state = head.state.clone();
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

    fn compute_state(&self, changes: &HashMap<Vec<u8>, Vec<u8>>, old_state: &State) -> State {
        let capacity = changes
            .iter()
            .fold(State::len(), |acc, (k, v)| acc + k.len() + v.len());

        let mut buf: Vec<u8> = Vec::with_capacity(capacity);

        buf.extend_from_slice(old_state.as_slice());

        for (k, v) in changes.iter() {
            buf.extend_from_slice(k);
            buf.extend_from_slice(v);
        }

        let bytes = DefaultKeyHasher::hash(&buf);
        debug_assert_eq!(bytes.len(), State::len());

        State::from(&bytes[..])
    }

    fn find_node<'a>(&'a self, state: &State) -> Option<&'a Box<Node>> {
        fn _find_node<'a>(node: Option<&'a Node>, state: &State) -> Option<&'a Box<Node>> {
            if node.is_none() {
                return None;
            }

            let node: &Node = node.unwrap();
            let boxed_prev = node.prev.as_ref();

            if node.state.as_slice() == state.as_slice() {
                return boxed_prev;
            }

            let prev: Option<&Node> = boxed_prev.map(|p| &**p);

            _find_node(prev, state)
        }

        let head = self.head.as_ref();

        if self.head.is_none() {
            return None;
        }

        _find_node(head, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_kv_empty() {
        let kv = StatefulKV::new();
        assert_eq!(kv.state, State::empty());
    }

    #[test]
    fn mock_kv_single_state() {
        let mut kv = StatefulKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);

        let changes = vec![(&k1[..], &v1[..]), (&k2[..], &v2[..])];
        kv.store(&changes[..]);

        assert_ne!(kv.state, State::empty());

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
