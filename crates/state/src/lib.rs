//! Global State implementation for the SVM.

// TODO: Add missing documentation.
#![warn(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

mod error;

use sqlx::SqlitePool;

use std::{collections::HashMap, convert::TryInto};

use svm_hash::{Blake3Hasher, Hasher};

pub use error::{GlobalStateError, Result};

/// Every commit
pub type Commit = [u8; 32];

const SQL_SCHEMA: &str = include_str!("resources/schema.sql");

fn hash_key_value_pair(key_hash: &[u8; 32], value: &[u8]) -> [u8; 32] {
    let mut hasher = Blake3Hasher::default();
    hasher.update(key_hash);
    hasher.update(value);
    hasher.finalize()
}

fn xor_fingerprint(sig_1: &mut [u8; 32], sig_2: &[u8; 32]) {
    for (a, b) in sig_1.iter_mut().zip(sig_2) {
        *a ^= *b;
    }
}

const ZERO_FINGERPRINT: [u8; 32] = [0; 32];

/// Some external resources:
///
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
    /// Creates a new [`GlobalState`] persisted by the given SQLite instance.
    pub async fn new(sqlite_uri: &str) -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(sqlite_uri).await?;
        let mut gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            next_commit_id: 1,
        };
        gs.initialize().await?;
        Ok(gs)
    }

    pub async fn initialize(&mut self) -> Result<()> {
        // When initializing the database, we must look for the most recent
        // commit. If not present, that means the database is pristine and we
        // must create a new commit with an empty fingerprint.
        let max_commit_id: Option<i64> = sqlx::query!(
            r#"
                SELECT "id"
                FROM "commits"
                ORDER BY "id" DESC
                LIMIT 1
            "#
        )
        .fetch_optional(&self.sqlite)
        .await?
        .map(|record| record.id);

        match max_commit_id {
            Some(id) => {
                self.next_commit_id = id;
            }
            None => {
                self.next_commit_id = 1;
                self.create_commit(ZERO_FINGERPRINT).await?;
            }
        }

        Ok(())
    }

    /// Creates a new, empty [`GlobalState`] with no persisted state at all. All
    /// state will be kept in an in-memory SQLite database.
    pub async fn in_memory() -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(":memory:").await?;
        sqlx::query(SQL_SCHEMA).execute(&sqlite).await?;

        let mut gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            next_commit_id: 1,
        };
        gs.initialize().await?;
        Ok(gs)
    }

    /// Fetches the value associated with the Blake3 hash of `key`. See
    /// [`GlobalState::get_by_hash`] for more information.
    pub async fn get(&self, key: &[u8], commit: Option<&Commit>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, commit).await
    }

    /// Fetches the value associated with a Blake3 `hash`. If `commit`
    /// is `None`, the most recent value will be returned; otherwise, only the
    /// values present at the time of `commit` would be considered.
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
                            WHERE "fingerprint" = ?1
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

    /// Sets the `value` associated with the Blake3 hash of `key`. See
    /// [`GlobalState::upsert_by_hash`] for more information.
    pub async fn upsert<V>(&mut self, key: &[u8], value: V)
    where
        V: Into<Vec<u8>>,
    {
        let hash = Blake3Hasher::hash(key);
        self.upsert_by_hash(hash, value).await;
    }

    /// Sets the `value` associated with a Blake3 `hash`. This change will be "dirty" until
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

    /// Creates a new [`Commit`] with the given fingerprint.
    async fn create_commit(&mut self, fingerprint: Commit) -> Result<()> {
        let fingerprint = &fingerprint[..];
        sqlx::query!(
            r#"
                INSERT INTO "commits" ("id", "fingerprint")
                VALUES (?1, ?2)
            "#,
            self.next_commit_id,
            fingerprint
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

        let mut fingerprint = [0; 32];

        for change in dirty_changes {
            let old_value: Option<Vec<u8>> = self.get_by_hash(&change.0, None).await?;
            if let Some(old_value) = old_value {
                xor_fingerprint(
                    &mut fingerprint,
                    &hash_key_value_pair(&change.0, &old_value),
                );
            }
            xor_fingerprint(&mut fingerprint, &hash_key_value_pair(&change.0, &change.1));

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

        self.update_commit_fingerprint(&fingerprint).await
    }

    async fn commit_fingerprint(&self, commit_id: i64) -> Result<Commit> {
        let bytes = sqlx::query!(
            r#"
                SELECT "fingerprint"
                FROM "commits"
                WHERE "id" = ?1
            "#,
            commit_id
        )
        .fetch_one(&self.sqlite)
        .await?
        .fingerprint;
        Ok(bytes.try_into().unwrap())
    }

    async fn update_commit_fingerprint(
        &mut self,
        partial_fingerprint: &[u8; 32],
    ) -> Result<Commit> {
        let mut fingerprint: [u8; 32] = self.commit_fingerprint(self.next_commit_id).await?;
        xor_fingerprint(&mut fingerprint, partial_fingerprint);
        let fingerprint_bytes = &fingerprint[..];

        sqlx::query!(
            r#"
                UPDATE "commits"
                SET "fingerprint" = ?1
                WHERE "id" = ?2
            "#,
            fingerprint_bytes,
            self.next_commit_id
        )
        .execute(&self.sqlite)
        .await?;

        Ok(fingerprint)
    }

    /// Returns the [`Commit`] fingerprint of the last ever checkpoint; i.e.
    /// persisted changes without dirty changes.
    pub async fn current(&self) -> Result<Commit> {
        self.commit_fingerprint(self.next_commit_id).await
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
    async fn root_fingerprint_changes_after_inserts() -> bool {
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
