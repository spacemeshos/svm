use std::collections::HashMap;

use super::super::StatefulKVStore;

use svm_common::{DefaultKeyHasher, KeyHasher, State};
use svm_kv::traits::KVStore;

#[derive(Debug)]
struct Node {
    parent: State,

    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl Node {
    fn empty() -> Self {
        Self {
            data: HashMap::new(),
            parent: State::empty(),
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.get(key).map(|v| v.to_vec())
    }
}

/// `FakeKV` is a naive implementation for an in-memory key-value store.
/// It is also `State` aware, so it implements the `StatefulKVStore` trait.
///
/// Should be used only for testing and developement purposes.
pub struct FakeKV {
    head: State,

    refs: HashMap<State, Node>,
}

impl KVStore for FakeKV {
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
                None => state = &node.parent,
                Some(v) => return Some(v),
            }
        }
    }

    fn store(&mut self, changes: &[(&[u8], &[u8])]) {
        let changes: HashMap<_, _> = changes
            .iter()
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
            .collect();

        let old_state = &self.head;
        let new_state = self.compute_state(&changes, old_state);

        let mut node = Node::empty();
        node.data = changes;
        node.parent = old_state.clone();

        self.head = new_state.clone();
        self.refs.insert(new_state, node);
    }
}

impl StatefulKVStore for FakeKV {
    fn rewind(&mut self, state: &State) {
        self.head = state.clone();
    }

    #[must_use]
    fn head(&self) -> State {
        self.head.clone()
    }
}

impl FakeKV {
    /// New `FakeKV` initialized with no data.
    pub fn new() -> Self {
        Self {
            head: State::empty(),
            refs: HashMap::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! apply_changes {
        ($kv:ident, $( ($k:expr => $v:expr), )* ) => {{
            let changes = vec![$( (&$k[..], &$v[..]), )*];

            $kv.store(&changes[..]);

            $kv.head()
        }};
    }

    macro_rules! assert_no_keys {
        ($kv:ident, $($k:expr), *) => {{
            $(
                let v = $kv.get(&$k[..]);
                assert!(v.is_none());
             )*
        }};
    }

    macro_rules! assert_keys {
        ($kv:ident, $( ($k:expr => $v:expr), )* ) => {{
            $(
                let v = $kv.get(&$k[..]);
                assert_eq!(v.unwrap(), $v);
             )*
        }};
    }

    macro_rules! assert_transition {
        ($kv:ident, $s1:expr => $s2:expr) => {{
            let node2 = $kv.refs.get(&$s2).unwrap();

            assert_eq!(node2.parent.as_slice(), $s1.as_slice());
        }};
    }

    #[test]
    fn fake_kv_empty() {
        let kv = FakeKV::new();
        assert_eq!(kv.head, State::empty());
    }

    #[test]
    fn fake_kv_single_state() {
        let mut kv = FakeKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);

        apply_changes!(kv,
          (k1 => v1),
          (k2 => v2),
        );

        assert_keys!(kv,
          (k1 => v1),
          (k2 => v2),
        );
    }

    #[test]
    fn fake_kv_two_states() {
        let mut kv = FakeKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);
        let (k3, v3) = (b"ccc", vec![0x60, 0x70]);
        let (k4, v4) = (b"ddd", vec![0x80, 0x90]);

        let s1 = apply_changes!(kv,
          (k1 => v1),
          (k2 => v2),
        );

        let s2 = apply_changes!(kv,
          (k3 => v3),
          (k4 => v4),
        );

        assert_keys!(kv,
          (k1 => v1),
          (k2 => v2),
          (k3 => v3),
          (k4 => v4),
        );

        assert_transition!(kv, s1 => s2);
    }

    #[test]
    fn fake_kv_three_states() {
        let mut kv = FakeKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);
        let (k3, v3) = (b"ccc", vec![0x60, 0x70]);

        let s1 = apply_changes!(kv,
          (k1 => v1),
        );

        let s2 = apply_changes!(kv,
          (k2 => v2),
        );

        let s3 = apply_changes!(kv,
          (k3 => v3),
        );

        assert_keys!(kv,
          (k1 => v1),
          (k2 => v2),
          (k3 => v3),
        );

        assert_transition!(kv, s1 => s2);
        assert_transition!(kv, s2 => s3);
    }

    #[test]
    fn fake_kv_update_a_key_value() {
        let mut kv = FakeKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"aaa", vec![0x30, 0x40]);

        assert_eq!(k1, k2);

        let s1 = apply_changes!(kv,
          (k1 => v1),
        );

        let s2 = apply_changes!(kv,
          (k2 => v2),
        );

        kv.rewind(&s1);
        assert_keys!(kv,
          (k1 => v1),
        );

        kv.rewind(&s2);
        assert_keys!(kv,
          (k2 => v2),
        );
    }

    #[test]
    fn fake_kv_rewind() {
        let mut kv = FakeKV::new();

        let (k1, v1) = (b"aaa", vec![0x10, 0x20]);
        let (k2, v2) = (b"bbb", vec![0x30, 0x40, 0x50]);
        let (k3, v3) = (b"ccc", vec![0x60, 0x70]);
        let (k4, v4) = (b"aaa", vec![0x60, 0x70]);

        assert_eq!(k1, k4);

        let s1 = apply_changes!(kv,
          (k1 => v1),
        );

        let s2 = apply_changes!(kv,
          (k2 => v2),
        );

        let s3 = apply_changes!(kv,
          (k3 => v3),
          (k1 => v4),
        );

        kv.rewind(&s1);
        assert_keys!(kv, (k1 => v1),);
        assert_no_keys!(kv, k2, k3);

        kv.rewind(&s2);
        assert_keys!(kv, (k1 => v1), (k2 => v2),);
        assert_no_keys!(kv, k3);

        kv.rewind(&s3);
        assert_keys!(kv,
          (k1 => v4),
          (k2 => v2),
          (k3 => v3),
        );
    }
}
