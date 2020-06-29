use std::collections::HashMap;
use std::fmt;

use super::super::StatefulKV;

use svm_common::{fmt::fmt_hex, DefaultKeyHasher, KeyHasher};
use svm_types::State;

/// `FakeKV` is a naive implementation for an in-memory stateful key-value store.
///
/// Should be used only for testing and development purposes.
///
/// At any given point there are two pieces of data:
///
/// * `flushed` - represents a fake persisted data.
///
///  The data-structure is similar to how git tree works (in a very high-level).
///  The current `State` is pointed by `head` and each `Node` has its own data and a pointer
///  to the previous `State`.
///
///  So when searching for a key `K` we first look under the internal data stored by S_n',
///  If there's a matching value `V` we're done. Otherwise, we move to S_n' parent and so on.
///  If we reach the `S0` (zero state) then it means there is no value for key `K`.
///
///   `head`
///     ^
///     |
///     |
///  S_n' (last)  -------- parent -------->  S_n   -------- . . . -------->  S0 = 0...0 (first)
///     data                                 data                              data
///   (k1, v1')                            (k1, v1)                           (empty)                          
///   (k2, v2)                             (k4, v4)
///   (k3, v3)
///
/// * `journal` - a vector of unflushed changes.
///
///   Each vector item consists of a 2-item tuple.
///   The first tuple item holds the checkpoint `State`. This is optional since the last uncheckpointed-yet
///   changes has no checkpoint `State` yet. It will be determined only after finalizing the next checkpoint.
///   Besides that "un-checkpoint" yet one, all previous `State` have a value. (i.e they are `Some(State)`).
///   with a single checkpoint.
///
///   The second tuple item contains the changeset of the checkpoint. Each `Change` is a `(key, value)` tuple.
///
///
/// ## Looking for a key's value
///
/// First we search under the unflushed data - the `journal`.
/// We scan the journal in reverse order, from the current unfinalized checkpoint to the previous checkpoint and so on.
/// For each checkpoint we scan its entries list of changes in reverse order.
///
/// If we find a matching value we halt and return the found value.
/// If we've reached the end of the journal then we move to the `unflushed` (see detailed explanation above).
///
pub struct FakeKV {
    head: State,

    flushed_head: State,

    flushed: HashMap<State, Node>,

    journal: Vec<(Option<State>, Vec<Change>)>,
}

#[derive(Debug)]
struct Change(Vec<u8>, Vec<u8>);

struct Node {
    parent: State,

    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl Node {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.data.get(key).map(|v| v.to_vec())
    }
}

impl StatefulKV for FakeKV {
    #[must_use]
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.get_journal(key).or_else(|| self.get_flushed(key))
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key = key.to_vec();
        let value = value.to_vec();
        let change = Change(key, value);

        let (_, changes) = self.journal.last_mut().unwrap();
        changes.push(change);
    }

    fn discard(&mut self) {
        let (maybe_state, changes) = self.journal.last_mut().unwrap();
        matches!(maybe_state, None);

        changes.clear();
    }

    fn flush(&mut self) {
        let (_, changes) = self.journal.last_mut().unwrap();
        assert_eq!(changes.len(), 0);

        let n = self.journal.len();
        assert!(n > 0);

        let mut parent = self.flushed_head.clone();

        for (state, changes) in &self.journal[0..n - 1] {
            let node = self.make_node(parent.clone(), changes);

            let node_state = state.as_ref().unwrap().clone();
            self.flushed.insert(node_state, node);

            parent = state.clone().unwrap();
        }

        self.flushed_head = parent;

        self.journal = vec![(None, Vec::new())];

        self.assert_journal_empty();
    }

    #[must_use]
    fn checkpoint(&mut self) -> State {
        let (_, changes) = self.journal.last().unwrap();
        let new_state = self.compute_state(&changes);

        let (maybe_state, _) = self.journal.last_mut().unwrap();
        matches!(maybe_state, None);
        maybe_state.replace(new_state.clone());

        self.head = new_state.clone();
        self.journal.push((None, Vec::new()));

        new_state
    }

    fn rewind(&mut self, state: &State) {
        self.assert_journal_empty();

        self.head = state.clone();
        self.flushed_head = self.head();
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
            flushed_head: State::empty(),
            flushed: HashMap::new(),
            journal: vec![(None, Vec::new())],
        }
    }

    fn get_journal(&self, key: &[u8]) -> Option<Vec<u8>> {
        for (_state, changes) in self.journal.iter().rev() {
            for change in changes.iter().rev() {
                if change.0 == key {
                    let value = change.1.to_vec();
                    return Some(value);
                }
            }
        }

        None
    }

    fn get_flushed(&self, key: &[u8]) -> Option<Vec<u8>> {
        let mut state = &self.head;

        loop {
            if state.is_empty() {
                return None;
            }

            let node = self.flushed.get(&state).unwrap();

            match node.get(key) {
                None => state = &node.parent,
                Some(v) => return Some(v),
            }
        }
    }

    fn make_node(&self, parent: State, changes: &[Change]) -> Node {
        let data = changes
            .iter()
            .map(|c| {
                let k = c.0.to_vec();
                let v = c.1.to_vec();

                (k, v)
            })
            .collect();

        Node { parent, data }
    }

    fn compute_state(&self, changes: &[Change]) -> State {
        let capacity = changes.iter().fold(State::len(), |acc, change| {
            let k = &change.0;
            let v = &change.1;

            acc + k.len() + v.len()
        });

        let mut buf = Vec::with_capacity(capacity);

        buf.extend_from_slice(self.head.as_slice());

        for change in changes.iter() {
            buf.extend_from_slice(&change.0);
            buf.extend_from_slice(&change.1);
        }

        let bytes = DefaultKeyHasher::hash(&buf);
        assert_eq!(bytes.len(), State::len());

        State::from(&bytes[..])
    }

    #[allow(unused)]
    fn journal_state_transition(&self, state: &State) -> Option<State> {
        for (i, (checkpoint, _changes)) in self.journal.iter().enumerate() {
            match checkpoint {
                None => return None,
                Some(checkpoint) => {
                    if checkpoint.as_slice() == state.as_slice() {
                        let next = i + 1;

                        let (next_state, _) = &self.journal[next];

                        return next_state.as_ref().cloned();
                    }
                }
            }
        }

        unreachable!()
    }

    fn assert_journal_empty(&self) {
        assert_eq!(self.journal.len(), 1);

        let (maybe_state, changes) = self.journal.last().unwrap();

        assert_eq!(changes.len(), 0);
        matches!(maybe_state, None);
    }
}

impl fmt::Debug for FakeKV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flushed_buf = String::new();
        let mut journal_buf = String::new();

        self.fmt_flushed(&mut flushed_buf)?;
        self.fmt_journal(&mut journal_buf)?;

        f.debug_struct("FakeKV")
            .field("HEAD", &fmt_state(&self.head))
            .field("flushed", &flushed_buf)
            .field("joural", &journal_buf)
            .finish()
    }
}

impl FakeKV {
    fn fmt_flushed<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        let mut state = &self.head;

        while state.is_empty() == false {
            let node = self.flushed.get(&state).unwrap();

            write!(f, "state: {}", &fmt_state(state))?;

            for (k, v) in node.data.iter() {
                let k = fmt_hex(k, ", ");
                let v = fmt_hex(v, ", ");

                write!(f, "{} -> {}", k, v)?;
            }

            state = &node.parent;
        }

        Ok(())
    }

    fn fmt_journal<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        for (checkpoint, _changes) in self.journal.iter() {
            match checkpoint {
                Some(checkpoint) => write!(f, "CHECKPOINT: {}", fmt_state(checkpoint)),
                None => write!(f, "CHECKPOINT: WIP"),
            }?;
        }

        Ok(())
    }
}

impl Drop for FakeKV {
    fn drop(&mut self) {
        dbg!("Dropping `FakeKV`");
    }
}

fn fmt_state(state: &State) -> String {
    let bytes = &state.as_slice();

    fmt_hex(&bytes[0..6], "")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! apply_changes {
        ($kv:ident, $( ($k:expr => $v:expr), )* ) => {{
            let changes = vec![$( (&$k[..], &$v[..]), )*];

            for (k, v) in changes.iter() {
                $kv.set(k, v);
            }

            let state = $kv.checkpoint();

            $kv.flush();

            state
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
            match $kv.flushed.get(&$s2) {
                Some(node2) => {
                    assert_eq!(node2.parent.as_slice(), $s1.as_slice());
                }
                None => {
                    let s2 = $kv.journal_state_transition(&$s1).unwrap();

                    assert_eq!($s2.as_slice(), s2.as_slice());
                }
            }
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
