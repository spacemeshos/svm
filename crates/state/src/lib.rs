mod error;
mod trie_node;

use sqlx::SqlitePool;

use std::{collections::HashMap, convert::TryInto};

use svm_hash::{Blake3Hasher, Hasher};

pub use error::GlobalStateError;

pub type Commit = [u8; 32];

const SQL_SCHEMA: &str = include_str!("resources/schema.sql");

type Result<T> = std::result::Result<T, GlobalStateError>;

fn hash_key_value_pair(key_hash: &[u8; 32], value: &[u8]) -> [u8; 32] {
    let mut hasher = Blake3Hasher::default();
    hasher.update(key_hash);
    hasher.update(value);
    hasher.finalize()
}

fn xor_signature(sig_1: &mut [u8; 32], sig_2: &[u8; 32]) {
    for (a, b) in sig_1.iter_mut().zip(sig_2) {
        *a ^= *b;
    }
}

const ZERO_SIGNATURE: [u8; 32] = [0; 32];

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
    next_commit_id: i64,
    dirty_changes: HashMap<[u8; 32], Vec<u8>>,
}

impl GlobalState {
    /// Creates a new [`GlobalState`] persisted by the given SQLite and Redis
    /// instances. Initially, the
    pub async fn new(_redis_uri: &str, sqlite_uri: &str) -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(sqlite_uri).await?;
        let mut gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            next_commit_id: 1,
        };
        gs.create_commit(ZERO_SIGNATURE).await?;
        Ok(gs)
    }

    pub async fn in_memory() -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(":memory:").await?;
        sqlx::query(SQL_SCHEMA).execute(&sqlite).await?;

        let mut gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            next_commit_id: 1,
        };
        gs.create_commit(ZERO_SIGNATURE).await?;
        Ok(gs)
    }

    /// Fetches the value associated with `key`, once hashed with Blake3.
    pub async fn get(&self, key: &[u8], commit: Option<&Commit>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, commit).await
    }

    /// Fetches the value associated with a Blake3 `hash`.
    pub async fn get_by_hash(
        &self,
        hash: &[u8; 32],
        commit: Option<&Commit>,
    ) -> Result<Option<Vec<u8>>> {
        // We are given an explicit [`Commit`], so we must add that condition.
        if let Some(commit) = commit {
            let hash = &hash[..];
            let commit = &commit[..];
            let value_opt: Option<Vec<u8>> = sqlx::query!(
                r#"
                    SELECT "value"
                    FROM "values"
                    WHERE
                        "commit_id" <= (
                            SELECT "id"
                            FROM "commits"
                            WHERE "signature" = ?1
                        )
                        AND
                        "key_hash" = ?2
                "#,
                commit,
                hash,
            )
            .fetch_optional(&self.sqlite)
            .await?
            .map(|x| x.value);
            Ok(value_opt)
        } else if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.to_vec()))
        } else {
            // No explicit [`Commit`]! We simply must fetch the last record,
            // which is the most recent one.
            let hash = &hash[..];
            let value: Option<Vec<u8>> = sqlx::query!(
                r#"
                    SELECT "value"
                    FROM "values"
                    WHERE "key_hash" = ?1
                    ORDER BY "id" DESC
                    LIMIT 1
                "#,
                hash,
            )
            .fetch_optional(&self.sqlite)
            .await?
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
        let value = value.into();
        self.dirty_changes.entry(hash).or_insert(value);
    }

    pub async fn merkelize(&self) -> Option<Commit> {
        None
    }

    /// Creates a new [`Commit`] with the given signature.
    async fn create_commit(&mut self, signature: Commit) -> Result<()> {
        let signature = &signature[..];
        sqlx::query!(
            r#"
                INSERT INTO "commits" ("id", "signature")
                VALUES (?1, ?2)
            "#,
            self.next_commit_id,
            signature
        )
        .execute(&self.sqlite)
        .await?;
        Ok(())
    }

    pub async fn commit(&mut self) -> Result<Commit> {
        let commit = self.checkpoint().await?;
        self.next_commit_id += 1;
        self.create_commit(commit).await?;
        Ok(commit)
    }

    /// Persists all dirty changes from memory to disk. Even though persisted to
    /// disk, they still don't belong to any commit; look into
    /// [`GlobalState::commit`].
    pub async fn checkpoint(&mut self) -> Result<Commit> {
        let dirty_changes = std::mem::take(&mut self.dirty_changes);

        let mut signature = [0; 32];

        for change in dirty_changes {
            let old_value: Option<Vec<u8>> = self.get_by_hash(&change.0, None).await?;
            if let Some(old_value) = old_value {
                xor_signature(&mut signature, &hash_key_value_pair(&change.0, &old_value));
            }
            xor_signature(&mut signature, &hash_key_value_pair(&change.0, &change.1));

            let key_hash = &change.0[..];
            let value = &change.1[..];
            sqlx::query!(
                r#"
                    INSERT INTO "values" ("commit_id", "key_hash", "value")
                    VALUES (?1, ?2, ?3)
                "#,
                self.next_commit_id,
                key_hash,
                value
            )
            .execute(&self.sqlite)
            .await?;
        }

        self.update_commit_signature(&signature).await
    }

    async fn update_commit_signature(&mut self, partial_signature: &[u8; 32]) -> Result<Commit> {
        let old_signature_vec: Vec<u8> = sqlx::query!(
            r#"
                SELECT "signature"
                FROM "commits"
                WHERE "id" = ?1
            "#,
            self.next_commit_id
        )
        .fetch_one(&self.sqlite)
        .await?
        .signature;

        let mut signature: [u8; 32] = old_signature_vec.try_into().unwrap();
        xor_signature(&mut signature, partial_signature);
        let signature_bytes = &signature[..];

        sqlx::query!(
            r#"
                UPDATE "commits"
                SET "signature" = ?1
                WHERE "id" = ?2
            "#,
            signature_bytes,
            self.next_commit_id
        )
        .execute(&self.sqlite)
        .await?;

        Ok(signature)
    }

    /// Returns the current root hash in the form of a [`Commit`].
    pub fn current(&self) -> Commit {
        unimplemented!()
    }

    /// Returns the current root hash in the form of a [`Commit`].
    ///
    /// It returns an error in case there's any dirty changes: you must
    /// [`rollback`](GlobalState::rollback) first.
    pub async fn rewind(&mut self, _commit: &Commit) -> Result<()> {
        if self.dirty_changes.is_empty() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[quickcheck_async::tokio]
    async fn retrieval_after_dirty_changes(items: HashMap<String, String>) -> bool {
        let mut gs = GlobalState::in_memory().await.unwrap();

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
        let mut gs = GlobalState::in_memory().await.unwrap();

        for (key, value) in items.iter().filter(|kv| !kv.0.is_empty()) {
            gs.upsert(key.as_bytes(), value.as_bytes()).await;
        }

        gs.checkpoint().await.unwrap();

        for (key, value) in items.iter().filter(|kv| !kv.0.is_empty()) {
            let stored_value = gs.get(key.as_bytes(), None).await.unwrap().unwrap();
            if stored_value != value.as_bytes() {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn root_signature_changes_after_inserts() -> bool {
        let mut gs = GlobalState::in_memory().await.unwrap();

        gs.upsert(b"foo", "bar").await;
        let commit_1 = gs.commit().await.unwrap();

        gs.upsert(b"foo", "spam").await;
        let commit_2 = gs.commit().await.unwrap();

        gs.upsert(b"foo", "bar").await;
        let commit_3 = gs.commit().await.unwrap();

        commit_1 != commit_2 && commit_1 == commit_3
    }

    #[quickcheck_async::tokio]
    async fn get_fails_on_empty_global_state(key: Vec<u8>) -> bool {
        let gs = GlobalState::in_memory().await.unwrap();
        matches!(gs.get(&key, None).await, Ok(None))
    }

    #[quickcheck_async::tokio]
    async fn rewind_succeeds_when_empty(bytes: Vec<u8>) -> bool {
        let context = Blake3Hasher::hash(&bytes);
        let mut gs = GlobalState::in_memory().await.unwrap();

        gs.rewind(&context).await.is_ok()
    }

    #[quickcheck_async::tokio]
    async fn rewind_fails_after_insert(bytes: Vec<u8>, key: Vec<u8>, value: Vec<u8>) -> bool {
        let context = Blake3Hasher::hash(&bytes);
        let mut gs = GlobalState::in_memory().await.unwrap();

        gs.upsert(&key, value).await;
        gs.rewind(&context).await.is_err()
    }
}
