mod error;
mod trie_node;

use svm_hash::{Blake3Hasher, Hasher};

use sqlx::SqlitePool;

use std::collections::HashMap;

pub use error::GlobalStateError;
use trie_node::TrieNode;

pub type Commit = [u8; 32];

const SQL_SCHEMA: &str = include_str!("resources/gs-schema.sql");

type Result<T> = std::result::Result<T, GlobalStateError>;

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
pub struct GlobalState {
    sqlite: SqlitePool,
    commit: Option<Commit>,
    dirty_changes: HashMap<[u8; 32], Vec<u8>>,
}

struct MerkelizationWalk {}

impl GlobalState {
    /// Creates a new [`GlobalState`] persisted by the given SQLite and Redis
    /// instances. Initially, the
    pub async fn new(_redis_uri: &str, sqlite_uri: &str) -> Self {
        let sqlite = sqlx::SqlitePool::connect(sqlite_uri).await.unwrap();
        let gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            commit: None,
        };
        gs.initialize().await;
        gs
    }

    pub async fn in_memory() -> Self {
        let sqlite = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(SQL_SCHEMA).execute(&sqlite).await.unwrap();

        let gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            commit: None,
        };
        gs.initialize().await;
        gs
    }

    async fn initialize(&self) {
        sqlx::query!(
            r#"
                INSERT INTO "commits" ("id", "hash")
                VALUES
                    (1, NULL)
            "#
        )
        .execute(&self.sqlite)
        .await
        .unwrap();
    }

    /// Fetches the value associated with `key`, once hashed with Blake3.
    pub async fn get(&self, key: &[u8], commit: Option<&Commit>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, commit).await
    }

    async fn commit_id(&self, commit: &Commit) -> i64 {
        let commit = &commit[..];
        let record = sqlx::query!(
            r#"
                SELECT "id"
                FROM "commits"
                WHERE "hash" = ?1
                "#,
            commit
        )
        .fetch_one(&self.sqlite)
        .await
        .unwrap();
        record.id
    }

    async fn next_commit_id(&self) -> i64 {
        let max_id = sqlx::query!(
            r#"
                SELECT "id"
                FROM "commits"
                ORDER BY "id" DESC
                LIMIT 1
            "#
        )
        .fetch_one(&self.sqlite)
        .await
        .unwrap();
        max_id.id
    }

    /// Fetches the value associated with a Blake3 `hash`.
    pub async fn get_by_hash(
        &self,
        hash: &[u8; 32],
        commit: Option<&Commit>,
    ) -> Result<Option<Vec<u8>>> {
        if let Some(commit) = commit {
            let hash = &hash[..];
            let commit_hash = &commit[..];
            let value_opt: Option<Vec<u8>> = sqlx::query!(
                r#"
                    SELECT "value"
                    FROM "values"
                    WHERE
                        "commit_id" <= (
                            SELECT "id"
                            FROM "commits"
                            WHERE "hash" = ?1
                        )
                        AND
                        "key_hash" = ?2
                    ORDER BY "id" DESC LIMIT 1
                "#,
                commit_hash,
                hash,
            )
            .fetch_optional(&self.sqlite)
            .await
            .unwrap()
            .map(|x| x.value);
            Ok(value_opt)
        } else if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.to_vec()))
        } else {
            let hash = &hash[..];
            let value: Option<Vec<u8>> = sqlx::query!(
                r#"
                    SELECT "value"
                    FROM "values"
                    WHERE
                        "commit_id" = (
                            SELECT MAX("id")
                            FROM "commits"
                        )
                        AND
                        "key_hash" = ?1
                    LIMIT 1
                "#,
                hash,
            )
            .fetch_optional(&self.sqlite)
            .await
            .unwrap()
            .map(|record| record.value);
            Ok(value)
        }
    }

    /// Sets the `value` associated with `key`, once hashed with Blake3.
    pub async fn upsert<V>(&mut self, key: &[u8], value: V)
    where
        V: Into<Vec<u8>>,
    {
        let hash = Blake3Hasher::hash(key);
        self.upsert_by_hash(hash, value).await;
    }

    /// Sets the `value` associated with a Blake3 `hash`.
    pub async fn upsert_by_hash<V>(&mut self, hash: [u8; 32], value: V)
    where
        V: Into<Vec<u8>>,
    {
        self.dirty_changes.entry(hash).or_insert(value.into());
    }

    pub async fn merkelize(&self) -> Option<Commit> {
        None
    }

    pub async fn commit(&mut self) -> Result<[u8; 32]> {
        self.checkpoint().await?;
        let commit_id = self.next_commit_id().await;
        let commit_hash = [0; 32]; // FIXME
        sqlx::query!(
            r#"
                INSERT INTO "commits" ("id", "hash")
                VALUES (?1, NULL)
            "#,
            commit_id,
        )
        .execute(&self.sqlite)
        .await
        .unwrap();
        Ok(commit_hash)
    }

    /// Persists all dirty changes from memory to disk. Even though persisted to
    /// disk, they still don't belong to any commit; look into
    /// [`GlobalState::commit`].
    pub async fn checkpoint(&mut self) -> Result<()> {
        let dirty_changes = std::mem::take(&mut self.dirty_changes);
        let commit_id = self.next_commit_id().await;

        for change in dirty_changes {
            let key_hash = &change.0[..];
            let value = &change.1[..];

            let num_rows_affected = sqlx::query!(
                r#"
                    INSERT INTO "values" ("commit_id", "key_hash", "value")
                    VALUES (?1, ?2, ?3)
                "#,
                commit_id,
                key_hash,
                value
            )
            .execute(&self.sqlite)
            .await
            .unwrap()
            .rows_affected();

            assert_eq!(num_rows_affected, 1);
        }

        Ok(())
    }

    /// Returns the current root hash in the form of a [`Commit`].
    pub fn current(&self) -> Option<Commit> {
        self.commit
    }

    /// Returns the current root hash in the form of a [`Commit`].
    ///
    /// It returns an error in case there's any dirty changes: you must
    /// [`rollback`](GlobalState::rollback) first.
    pub async fn rewind(&mut self, commit: &Commit) -> Result<()> {
        if self.dirty_changes.is_empty() {
            self.commit = Some(*commit);
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
    use super::*;

    #[quickcheck_async::tokio]
    async fn retrieval_after_dirty_changes(items: HashMap<String, String>) -> bool {
        let mut gs = GlobalState::in_memory().await;

        for (key, value) in items.iter() {
            gs.upsert(key.as_bytes(), value.as_bytes()).await;
        }

        for (key, value) in items {
            let stored_value = gs.get(key.as_bytes(), None).await.unwrap().unwrap();
            if stored_value != value.as_bytes() {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn retrieval_after_checkpoint(items: HashMap<String, String>) -> bool {
        let mut gs = GlobalState::in_memory().await;

        assert_eq!(gs.next_commit_id().await, 1);
        for (key, value) in items.iter().filter(|kv| !kv.0.is_empty()) {
            gs.upsert(key.as_bytes(), value.as_bytes()).await;
        }

        gs.checkpoint().await.unwrap();
        assert_eq!(gs.next_commit_id().await, 1);

        for (key, value) in items.iter().filter(|kv| !kv.0.is_empty()) {
            let stored_value = gs.get(key.as_bytes(), None).await.unwrap().unwrap();
            println!("VALUES {:?} {:?} {:?}", key, value, stored_value);
            if stored_value != value.as_bytes() {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn root_hash_changes_after_inserts(key: Vec<u8>, value: Vec<u8>) -> bool {
        true // TODO
    }

    #[quickcheck_async::tokio]
    async fn get_fails_on_empty_global_state(key: Vec<u8>) -> bool {
        let mut gs = GlobalState::in_memory().await;
        matches!(gs.get(&key, None).await, Ok(None))
    }

    #[quickcheck_async::tokio]
    async fn rewind_succeeds_when_empty(bytes: Vec<u8>) -> bool {
        let context = Blake3Hasher::hash(&bytes);
        let mut gs = GlobalState::in_memory().await;

        gs.rewind(&context).await.is_ok()
    }

    #[quickcheck_async::tokio]
    async fn rewind_fails_after_insert(bytes: Vec<u8>, key: Vec<u8>, value: Vec<u8>) -> bool {
        let context = Blake3Hasher::hash(&bytes);
        let mut gs = GlobalState::in_memory().await;
        gs.upsert(&key, value).await;

        gs.rewind(&context).await.is_err()
    }
}
