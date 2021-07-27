use blake3::Hash;
use thiserror::Error;
use trie_db::node::{NibbleSlicePlan, NodeHandlePlan, NodePlan};
use trie_db::triedbmut::TrieDBMut;
use trie_db::Hasher;

use std::convert::TryInto;
use std::fmt::Display;

struct TrieLayout;

impl trie_db::TrieLayout for TrieLayout {
    type Hash = Blake3Hasher;
    type Codec = TrieCodec;

    const USE_EXTENSION: bool = false;
    const ALLOW_EMPTY: bool = false;
}

struct Blake3Hasher;

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

impl Hasher for Blake3Hasher {
    type Out = [u8; 32];
    type StdHasher = Blake3StdHasher;

    const LENGTH: usize = 32;

    fn hash(x: &[u8]) -> Self::Out {
        (*blake3::hash(x).as_bytes())
            .try_into()
            .expect("Invalid hash size!")
    }
}

const NULL_NODE_ENCODING: &[u8] = b"e";

struct TrieCodec;

#[derive(Clone, Debug, Error)]
pub struct CodecError {}

impl Display for CodecError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl trie_db::NodeCodec for TrieCodec {
    type Error = CodecError;
    type HashOut = [u8; 32];

    fn hashed_null_node() -> Self::HashOut {
        Blake3Hasher::hash(NULL_NODE_ENCODING)
    }

    fn empty_node() -> &'static [u8] {
        NULL_NODE_ENCODING
    }

    fn decode_plan(data: &[u8]) -> Result<NodePlan, Self::Error> {
        if data.is_empty() {
            return Err(Self::Error {});
        }
        match data[0] {
            b'e' => Ok(NodePlan::Empty),
            b'b' => Ok(NodePlan::Branch {
                value: None,
                children: [None; 16],
            }),
            b'l' => Ok(NodePlan::Leaf {
                partial: 0,
                value: 0..0,
            }),
            b'x' => Ok(NodePlan::Extension {
                partial: NibbleSlicePlan::new(0..0, 0),
                child: NodeHandlePlan::Hash(0..0),
            }),
            _ => Err(Self::Error {}),
        }
    }

    fn leaf_node(partial: trie_db::Partial, value: &[u8]) -> Vec<u8> {
        let mut node = b"l".to_vec();
        node
    }

    fn extension_node(
        partial: impl Iterator<Item = u8>,
        number_nibble: usize,
        child_ref: trie_db::ChildReference<Self::HashOut>,
    ) -> Vec<u8> {
        let mut node = b"x".to_vec();
        for byte in partial {
            node.push(byte);
        }
        node
    }

    fn is_empty_node(data: &[u8]) -> bool {
        data == Self::empty_node()
    }

    fn branch_node(
        children: impl Iterator<
            Item = impl std::borrow::Borrow<Option<trie_db::ChildReference<Self::HashOut>>>,
        >,
        value: Option<&[u8]>,
    ) -> Vec<u8> {
        let mut node = b"b".to_vec();
        node
    }

    fn branch_node_nibbled(
        partial: impl Iterator<Item = u8>,
        number_nibble: usize,
        children: impl Iterator<
            Item = impl std::borrow::Borrow<Option<trie_db::ChildReference<Self::HashOut>>>,
        >,
        value: Option<&[u8]>,
    ) -> Vec<u8> {
        let mut node = b"n".to_vec();
        for byte in partial {
            node.push(byte);
        }
        node
    }
}

pub struct GlobalState<'a> {
    trie: TrieDBMut<'a, TrieLayout>,
}

impl<'a> GlobalState<'a> {
    pub fn deploy(&mut self) {}

    pub fn call(&mut self) {}

    pub fn spawn(&mut self) {}
}

impl Default for GlobalState {
    fn default() -> Self {
        unimplemented!()
    }
}

pub enum Item {
    Template { code_hash: Hash },
    Account { balance: u64, storage_hash: Hash },
}
