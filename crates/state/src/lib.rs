mod backend;
mod error;
mod trie_node;

use blake3::Hash;
use svm_hash::{Blake3Hasher, Hasher};

use std::collections::HashMap;
use std::convert::TryInto;

use trie_node::TrieNode;

pub use backend::DbBackend;
pub use error::GlobalStateError;

pub type Context = [u8; 32];

type Result<T, B> = std::result::Result<T, GlobalStateError<<B as DbBackend>::Error>>;

#[derive(Clone, Debug, Default)]
struct Blake3StdHasher(blake3::Hasher);

impl std::hash::Hasher for Blake3StdHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn finish(&self) -> u64 {
        let mut hash = [0; 8];
        self.0.finalize_xof().fill(&mut hash);
        u64::from_be_bytes(hash)
    }
}

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
    dirty_changes: HashMap<[u8; 32], [u8; 32]>,
}

#[inline]
fn dumb_longest_prefix(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
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

    fn is_insert(&self) -> bool {
        !self.is_update()
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

impl<B> GlobalState<B>
where
    B: backend::DbBackend,
{
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            dirty_changes: HashMap::new(),
            root: [0; 32],
        }
    }

    pub fn get_hash(&mut self, hash: &[u8; 32]) -> Result<Option<Vec<u8>>, B> {
        let mut node_hash = self.root;
        let mut hash_leftover = &hash[..];
        let mut i = 0;

        while i < 256 {
            let node_bytes = self
                .backend
                .get(&node_hash[..])
                .unwrap()
                .ok_or(GlobalStateError::InvalidItem)?;
            let node = TrieNode::decode(&node_bytes).unwrap();

            match node {
                TrieNode::Branch {
                    prefix,
                    children_hashes,
                } => {
                    if hash_leftover.starts_with(prefix) {
                        // Let's discard the part of the prefix that we just
                        // matched against and keep searching.
                        hash_leftover = &hash_leftover[prefix.len()..];
                        node_hash = *children_hashes[hash_leftover[0] as usize];
                        hash_leftover = &hash_leftover[1..];
                    } else {
                        return Ok(None);
                    }
                }
                TrieNode::Leaf { prefix, value } => {
                    // After we finally reach a leaf, there's not much left to
                    // do. Either the remaining prefix matches and we have a
                    // successful match, or we don't.
                    if hash_leftover == prefix {
                        return Ok(Some(value.to_vec()));
                    } else {
                        return Ok(None);
                    }
                }
            }

            i += 1;
        }

        Err(GlobalStateError::Cyclic)
    }

    /// Sets the `value` of `key`.
    pub fn upsert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, B> {
        let hash = blake3::hash(key);

        if self.root == NULL_HASH {
            self.root = *hash.as_bytes();
        }

        self.upsert_hash(hash.as_bytes(), value)
    }

    pub fn upsert_hash(&mut self, hash: &[u8; 32], value: &[u8]) -> Result<Option<Vec<u8>>, B> {
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
            walk.push(node_hash.to_vec());

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
                        });
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
                    });
                }
            }

            i += 1;
        }

        for node_hash in walk.iter().rev() {
            let old_value = self
                .get(&node_hash[..].try_into().unwrap())
                .unwrap()
                .unwrap();
        }

        return Err(GlobalStateError::Cyclic);
    }

    pub fn get(&self, hash: &[u8; 32]) -> Result<Option<Vec<u8>>, B> {
        if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.to_vec()))
        } else {
            self.backend
                .get(hash)
                .map_err(|e| GlobalStateError::Backend(e))
        }
    }

    fn update_branch(&mut self, details: AddLeafAfterBranch) -> Result<(), B> {
        let leaf = details.new_leaf();
        let leaf_hash = leaf.hash::<Blake3Hasher>();
        let old_branch = details.old_branch();
        let new_branch = details.new_branch(details.old_branch_hash, &leaf_hash);

        self.upsert_node(old_branch, details.old_branch_hash)?;
        self.upsert_node(leaf, details.new_leaf_hash)?;
        self.upsert_node(new_branch, details.old_branch_hash)?;

        Ok(())
    }

    fn upsert_node(&mut self, node: TrieNode, hash: &[u8; 32]) -> Result<(), B> {
        self.upsert(hash, &node.encode())?;
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

    pub fn commit(&mut self) -> [u8; 32] {
        let dirty_changes = std::mem::take(&mut self.dirty_changes);
        //for change in dirty_changes {
        //    self.upsert(&change.0, &change.1).expect("Error inserting");
        //}
        self.current()
    }

    pub fn checkpoint(&mut self) {
        unimplemented!()
    }

    pub fn current(&self) -> [u8; 32] {
        unimplemented!("Root hash")
    }

    pub fn rewind(&mut self, context: &Context) {
        // Delete *all* work from future [`Context`]'s.
    }

    pub fn rollback(&mut self) {
        self.dirty_changes.clear();
    }
}

pub enum Item {
    Template { code_hash: Hash },
    Account { balance: u64, storage_hash: Hash },
}

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn insert_then_get(items: Vec<(String, String)>) -> bool {
        let mut gs = GlobalState::new(HashMap::new());

        for (key, value) in items.iter() {
            gs.upsert(key.as_bytes(), value.as_bytes()).unwrap();
        }

        for (key, value) in items {
            if gs.get(key.as_bytes().try_into().unwrap()).unwrap()
                != Some(value.as_bytes().to_vec())
            {
                return false;
            }
        }

        true
    }

    #[quickcheck]
    fn root_hash_changes_after_inserts(key: Vec<u8>, value: Vec<u8>) -> bool {
        true // TODO
    }
}
