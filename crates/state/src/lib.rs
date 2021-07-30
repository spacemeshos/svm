mod backend;
mod error;
mod trie_node;

use svm_hash::{Blake3Hasher, Hasher};

use std::collections::HashMap;
use std::convert::TryInto;

pub use backend::DbBackend;
pub use error::GlobalStateError;
use trie_node::TrieNode;

pub type Context = [u8; 32];

type Result<T, B> = std::result::Result<T, GlobalStateError<<B as DbBackend>::Error>>;

const NULL_HASH: [u8; 32] = [0; 32];

/// * https://www.programmersought.com/article/33363860077/
/// * https://www.youtube.com/watch?v=oEpY4NkkeYQ
///
/// Lists of namespaces within [`GlobalState`]:
///
/// * 't': templates
///   * ''
/// * 'a': accounts
///   * 'b': balance
///   * 'n': nonce
///   * 'a': address
pub struct GlobalState<B>
where
    B: DbBackend,
{
    backend: B,
    root: Context,
    dirty_changes: HashMap<[u8; 32], Vec<u8>>,
}

impl<B> GlobalState<B>
where
    B: backend::DbBackend,
{
    /// Creates a new [`GlobalState`] persisted by `backend`.
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            dirty_changes: HashMap::new(),
            root: [0; 32],
        }
    }

    /// Fetches the value associated with `key`, once hashed with Blake3.
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, B> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash)
    }

    /// Fetches the value associated with a Blake3 `hash`.
    pub fn get_by_hash(&self, hash: &[u8; 32]) -> Result<Option<Vec<u8>>, B> {
        if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.to_vec()))
        } else {
            self.backend.get(hash)
        }
    }

    /// Sets the `value` associated with `key`, once hashed with Blake3.
    pub fn upsert<V>(&mut self, key: &[u8], value: V)
    where
        V: Into<Vec<u8>>,
    {
        let hash = Blake3Hasher::hash(key);
        self.upsert_by_hash(hash, value);
    }

    /// Sets the `value` associated with a Blake3 `hash`.
    pub fn upsert_by_hash<V>(&mut self, hash: [u8; 32], value: V)
    where
        V: Into<Vec<u8>>,
    {
        self.dirty_changes.entry(hash).or_insert(value.into());
    }

    pub fn persist_operation(
        &mut self,
        hash: &[u8; 32],
        value: &[u8],
    ) -> Result<Option<Vec<u8>>, B> {
        if self.root == NULL_HASH {
            let leaf = TrieNode::Leaf {
                prefix: hash,
                value,
            };
            let hash = leaf.hash::<Blake3Hasher>();
            self.upsert_node(leaf, &hash)?;
            return Ok(None);
        }

        let mut node_hash = self.root;
        let hash_leftover = hash;
        let mut i = 0;
        let mut walk = vec![];

        while i < 256 {
            let node_bytes_vec = self.get(&node_hash)?.ok_or(GlobalStateError::InvalidItem)?;
            let node_bytes = &node_bytes_vec[..];
            let node = TrieNode::decode(node_bytes).unwrap();
            walk.push(node_hash);

            match node {
                TrieNode::Branch {
                    prefix,
                    children_hashes,
                } => {
                    let common_prefix_len = dumb_longest_prefix(hash_leftover, prefix);

                    if common_prefix_len == prefix.len() {
                        node_hash = *children_hashes[hash_leftover[0] as usize];
                    } else {
                        self.update_branch(AddLeafAfterBranch {
                            old_branch: node,
                            old_branch_hash: &node_hash,
                            new_leaf_hash: hash,
                            new_leaf_value: value,
                            prefix: common_prefix_len,
                        })?;
                    }
                }
                TrieNode::Leaf {
                    prefix,
                    value: leaf_value,
                } => {
                    self.update_leaf(SplitLeafIntoTwo {
                        old_hash: &node_hash,
                        old_value: leaf_value,
                        new_hash: hash.try_into().unwrap(),
                        new_value: value,
                        prefix_len_before_old_leaf: hash.len() - hash_leftover.len(),
                        common_prefix_len_in_new_branch: 0,
                    })?;
                }
            }

            i += 1;
        }

        for node_hash in walk.iter().rev() {
            let old_value = self.get_by_hash(node_hash).unwrap().unwrap();
        }

        return Err(GlobalStateError::Cyclic);
    }

    fn update_branch(&mut self, details: AddLeafAfterBranch) -> Result<(), B> {
        let leaf = details.new_leaf();
        let leaf_hash = leaf.hash::<Blake3Hasher>();
        let old_branch = details.old_branch();
        let new_branch = details.new_branch(details.old_branch_hash, &leaf_hash);

        self.backend
            .upsert(details.old_branch_hash, &old_branch.encode())?;
        self.backend.upsert(details.new_leaf_hash, &leaf.encode())?;
        self.backend
            .upsert(details.old_branch_hash, &new_branch.encode())?;

        Ok(())
    }

    fn update_leaf(&mut self, details: SplitLeafIntoTwo) -> Result<Option<Vec<u8>>, B> {
        if details.is_update() {
            let old_value = details.new_value.to_vec();
            self.backend
                .upsert(details.old_hash, details.new_value)
                .unwrap();
            return Ok(Some(old_value));
        } else {
            self.upsert_node(details.old_leaf(), details.old_hash)
                .unwrap();
            self.upsert_node(details.new_leaf(), details.new_hash)
                .unwrap();

            let branch = TrieNode::Branch {
                prefix: details.branch_prefix(),
                children_hashes: [
                    &details.old_leaf().hash::<Blake3Hasher>(),
                    &details.new_leaf().hash::<Blake3Hasher>(),
                ],
            };

            self.upsert_node(branch, &branch.hash::<Blake3Hasher>())
                .unwrap();

            return Ok(None);
        }
    }

    fn upsert_node(&mut self, node: TrieNode, hash: &[u8; 32]) -> Result<(), B> {
        self.backend.upsert(hash, &node.encode())?;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<[u8; 32], B> {
        self.checkpoint()?;
        Ok(self.current())
    }

    /// Persists all dirty changes from memory to disk.
    pub fn checkpoint(&mut self) -> Result<(), B> {
        let dirty_changes = std::mem::take(&mut self.dirty_changes);
        for change in dirty_changes {
            self.persist_operation(&change.0, &change.1)?;
        }
        Ok(())
    }

    /// Returns the current root hash in the form of a [`Context`].
    pub fn current(&self) -> Context {
        self.root
    }

    /// Returns the current root hash in the form of a [`Context`].
    pub fn rewind(&mut self, context: &Context) -> Result<(), B> {
        if self.dirty_changes.is_empty() {
            self.root = *context;
            Ok(())
        } else {
            Err(GlobalStateError::DirtyChanges)
        }
    }

    /// Erases all dirty changes from memory. Persisted data is left untouched.
    pub fn rollback(&mut self) {
        self.dirty_changes.clear();
    }
}

/// A collection of fields that we need when splitting a [`TrieNode::Leaf`]
/// into a [`TrieNode::Branch`] with two [`TrieNode::Leaf`]'s (i.e. adding a
/// [`TrieNode::Leaf`]).
///
/// # Examples
///
/// Adding a `00110100` key:
///
/// ```text
/// B: 00110...
///   0. L: 000
///   1. L: 111
/// ```
///
/// Result:
///
/// ```text
/// B: 00110...
///   0. L: 000
///   1. B: 1...
///     0. L: 00
///     1. L: 11
/// ```
struct SplitLeafIntoTwo<'a> {
    old_hash: &'a [u8; 32],
    old_value: &'a [u8],
    new_hash: &'a [u8; 32],
    new_value: &'a [u8],
    prefix_len_before_old_leaf: usize,
    common_prefix_len_in_new_branch: usize,
}

impl<'a> SplitLeafIntoTwo<'a> {
    fn is_update(&self) -> bool {
        // If we have a full match, that means that we're updating
        // a value rather than inserting a new one.
        self.old_hash == self.new_hash
    }

    fn branch_prefix(&self) -> &'a [u8] {
        &self.old_hash[self.prefix_len_before_old_leaf..][..self.common_prefix_len_in_new_branch]
    }

    fn old_leaf(&self) -> TrieNode<'a> {
        TrieNode::Leaf {
            prefix: &[],
            value: self.old_value,
        }
    }

    fn new_leaf(&self) -> TrieNode<'a> {
        TrieNode::Leaf {
            prefix: &[],
            value: self.new_value,
        }
    }
}

/// # Examples
///
/// Adding a `00110110` key:
///
/// ```text
/// B: 00110...
///   0. L: 000
///   1. B: 10...
///     0. L: 0
///     1. L: 1
/// ```
///
/// Result:
///
/// ```text
/// B: 00110...
///   0. L: 000
///   1. B: 1...
///     0. B: 0...
///       0. L: 0
///       1. L: 1
///     1. L: 10
/// ```
struct AddLeafAfterBranch<'a> {
    old_branch: TrieNode<'a>,
    old_branch_hash: &'a [u8; 32],
    new_leaf_hash: &'a [u8; 32],
    new_leaf_value: &'a [u8],
    prefix: usize,
}

impl<'a> AddLeafAfterBranch<'a> {
    fn new_leaf(&self) -> TrieNode {
        TrieNode::Leaf {
            prefix: &[],
            value: self.new_leaf_value,
        }
    }

    fn old_branch(&self) -> TrieNode {
        match self.old_branch {
            TrieNode::Branch {
                children_hashes,
                prefix,
            } => TrieNode::Branch {
                children_hashes,
                prefix,
            },
            _ => panic!(),
        }
    }

    fn new_branch(&self, old_branch_hash: &'a [u8; 32], new_leaf_hash: &'a [u8; 32]) -> TrieNode {
        TrieNode::Branch {
            children_hashes: [old_branch_hash, new_leaf_hash],
            prefix: &[],
        }
    }
}

#[inline]
fn dumb_longest_prefix(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn insert_then_get(items: Vec<(String, String)>) -> bool {
        let mut gs = GlobalState::new(HashMap::new());

        for (key, value) in items.iter() {
            gs.upsert(key.as_bytes(), value.as_bytes());
        }

        for (key, value) in items {
            let expected = Some(value.as_bytes().to_vec());
            if !matches!(gs.get(key.as_bytes().try_into().unwrap()), expected) {
                return false;
            }
        }

        true
    }

    #[quickcheck]
    fn root_hash_changes_after_inserts(key: Vec<u8>, value: Vec<u8>) -> bool {
        true // TODO
    }

    #[quickcheck]
    fn get_fails_on_empty_global_state(key: Vec<u8>) -> bool {
        let gs = GlobalState::new(HashMap::new());
        matches!(gs.get(&key), Ok(None))
    }

    #[quickcheck]
    fn rewind_succeeds_when_empty(bytes: Vec<u8>) -> bool {
        let context = Blake3Hasher::hash(&bytes);
        let mut gs = GlobalState::new(HashMap::new());

        gs.rewind(&context).is_ok()
    }

    #[quickcheck]
    fn rewind_fails_after_insert(bytes: Vec<u8>, key: Vec<u8>, value: Vec<u8>) -> bool {
        let context = Blake3Hasher::hash(&bytes);
        let mut gs = GlobalState::new(HashMap::new());
        gs.upsert(&key, value);

        gs.rewind(&context).is_err()
    }
}
