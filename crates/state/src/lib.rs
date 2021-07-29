mod backend;
mod error;
mod trie_node;

use blake3::Hash;

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
    dirty_changes: HashMap<[u8; 32], [u8; 32], Blake3StdHasher>,
    current_context: Context,
}

#[inline]
fn dumb_longest_prefix(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}

impl<B> GlobalState<B>
where
    B: backend::DbBackend,
{
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            dirty_changes: HashMap::with_hasher(Blake3StdHasher::default()),
            current_context: [0; 32],
        }
    }

    pub fn get(&mut self, context: &Context, key: &[u8]) -> Result<Option<Vec<u8>>, B> {
        let hash = blake3::hash(key);
        self.get_hash(context, hash.as_bytes())
    }

    pub fn get_hash(&mut self, context: &Context, hash: &[u8]) -> Result<Option<Vec<u8>>, B> {
        assert!(hash.len() == 32, "Invalid hash length. Must be 32 bytes.");

        let mut node_hash = &*context;
        let mut hash_leftover = hash;
        let mut i = 0;

        while i < 256 {
            let node_bytes = self
                .backend
                .get(node_hash)
                .unwrap()
                .ok_or(GlobalStateError::InvalidItem)?;
            let node = TrieNode::decode(&node_bytes).unwrap();

            match node {
                TrieNode::Branch {
                    prefix,
                    children_hashes: [child_l_hash, child_r_hash],
                } => {
                    // The current branch node is only valid for nodes that have
                    // a specific common prefix. Anything else results in an
                    // unsuccessful search.
                    if hash_leftover.starts_with(prefix) && hash_leftover != prefix {
                        // Let's discard the part of the prefix that we just
                        // matched against.
                        hash_leftover = &hash_leftover[prefix.len()..];
                    } else {
                        return Ok(None);
                    }

                    // Let's keep on searching.
                    if hash_leftover[0] == 0 {
                        node_hash = child_l_hash.try_into().unwrap();
                    } else {
                        node_hash = child_r_hash.try_into().unwrap();
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

        Ok(None)
    }

    /// Sets the `value` of `key` within a [`Context`]. Different [`Context`]'s
    /// will give different mappings.
    pub fn upsert(
        &mut self,
        context: &Context,
        key: &[u8],
        value: &[u8],
    ) -> Result<Option<Vec<u8>>, B> {
        let hash = blake3::hash(key);
        self.upsert_hash(context, hash.as_bytes(), value)
    }

    pub fn upsert_hash(
        &mut self,
        context: &Context,
        hash: &[u8; 32],
        value: &[u8],
    ) -> Result<Option<Vec<u8>>, B> {
        struct UpsertDetails<'a> {
            hash: &'a [u8; 32],
            value: &'a [u8],
        };

        let mut node_hash = &*context;
        let mut hash_leftover = hash;
        let mut i = 0;
        let mut walk = vec![];

        while i < 256 {
            let node_bytes = self
                .backend
                .get(node_hash)
                .unwrap()
                .ok_or(GlobalStateError::InvalidItem)?;
            let node = TrieNode::decode(&node_bytes).unwrap();
            walk.push(node_hash.to_vec());

            match node {
                TrieNode::Branch {
                    prefix,
                    children_hashes: [child_l_hash, child_r_hash],
                } => {
                    if hash_leftover.starts_with(prefix) && hash_leftover != prefix {
                        self.update_branch();
                    } else {
                        return Ok(None);
                    }
                    if hash_leftover[0] == 0 {
                        node_hash = child_l_hash.try_into().unwrap();
                    } else {
                        node_hash = child_r_hash.try_into().unwrap();
                    }
                }
                TrieNode::Leaf {
                    prefix,
                    value: leaf_value,
                } => {
                    self.update_leaf(
                        node_hash,
                        leaf_value,
                        hash.try_into().unwrap(),
                        value,
                        prefix,
                        hash_leftover,
                    );
                }
            }

            i += 1;
        }

        for node_hash in walk.iter().rev() {
            let old_value = self.backend.get(&node_hash).unwrap().unwrap();
        }

        return Err(GlobalStateError::Cyclic);
    }

    fn update_branch(
        &mut self,
        context: &Context,
        prefix_len_before_branch: usize,
        common_prefix_in_branch: usize,
        hash_of_branch_node: &[u8; 32],
        branch_node: &TrieNode,
        leaf_hash: &[u8; 32],
        leaf_value: &[u8],
    ) -> Result<(), B> {
        debug_assert_eq!(hash_of_branch_node, branch_node.hash());

        let (prefix, children_hashes) = if let TrieNode::Branch {
            prefix,
            children_hashes,
        } = branch_node
        {
            (prefix, children_hashes)
        } else {
            panic!("Non-branch node");
        };

        let hash_leftover = &leaf_hash[prefix_len_before_branch..];
        let common_prefix = &prefix[..common_prefix_in_branch];

        let new_branch_node = TrieNode::Branch {
            prefix: common_prefix,
            children_hashes: [hash_of_old_branch_node, &[0; 32]],
        };

        let leaf = TrieNode::Leaf {
            prefix: hash_leftover,
            value: leaf_value,
        };

        self.backend
            .upsert(leaf_hash, &leaf.encode())
            .map_err(|e| GlobalStateError::Backend(e))?;

        Ok(())
    }

    fn update_leaf(
        &mut self,
        leaf_hash: &[u8; 32],
        leaf_value: &[u8],
        new_hash: &[u8; 32],
        new_value: &[u8],
        prefix: &[u8],
        hash_leftover: &[u8],
    ) -> Result<Option<Vec<u8>>, B> {
        debug_assert_eq!(prefix.len(), hash_leftover.len());

        // If we have a full match, that means that we're updating
        // a value rather than inserting a new one.
        let is_update = hash_leftover == prefix;

        if is_update {
            let old_value = leaf_value.to_vec();
            self.backend.upsert(new_hash, new_value).unwrap();
            return Ok(Some(old_value));
        } else {
            let common_prefix_len = dumb_longest_prefix(leaf_hash, new_hash);
            let common_prefix = &prefix[..common_prefix_len];
            let new_branch_node = TrieNode::Branch {
                prefix: common_prefix,
                children_hashes: [&[0; 32], &[0; 32]],
            };

            return Ok(None);
        }
    }

    pub fn commit(&mut self) -> [u8; 32] {
        let dirty_changes = std::mem::take(&mut self.dirty_changes);
        for change in dirty_changes {
            self.upsert(&change.0, &change.1).expect("Error inserting");
        }
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
    fn insert_then_get(key: Vec<u8>, value: Vec<u8>) -> bool {
        let trie = &mut Trie::<_, Blake3Hasher>::new(HashMap::default());
        println!("After inserting root, tree is {:?}", trie.backend);
        assert!(trie.upsert(&key, &value).unwrap().is_none());
        println!("After inserting root and node ,tree is {:?}", trie.backend);

        trie.get(&key).unwrap() == Some(value)
    }

    #[quickcheck]
    fn root_hash_changes_after_inserts(key: Vec<u8>, value: Vec<u8>) -> bool {
        true // TODO
    }
}
