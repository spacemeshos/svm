//! Global State implementation for the SVM.

// TODO: Add missing documentation.
#![warn(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

mod error;

use sqlx::SqlitePool;

use std::collections::HashMap;
use std::convert::TryInto;

use svm_hash::{Blake3Hasher, Hasher};

pub use error::{Result, StorageError};

/// Hashes as well as root fingerprints are 256 bits long.
pub type Fingerprint = [u8; 32];

type Changes = HashMap<Fingerprint, Vec<u8>>;

const SQL_SCHEMA: &str = include_str!("resources/schema.sql");
const INITIAL_COMMIT_ID: i64 = 1;

// When initializing [`Fingerprint`]'s on the database, we zero all bits. In
// memory, however, the initial [`Fingerprint`] is always filled with 1's.
const FINGERPRINT_ZEROS: Fingerprint = [0; 32];
const FINGERPRINT_ONES: Fingerprint = [std::u8::MAX; 32];

struct CurrentCommit {
    id: i64,
    changes: Changes,
    changes_xor_fingerprint: Fingerprint,
}

/// A SQLite-backed key-value store that supports root fingerprinting and
/// historical state queries.
pub struct Storage {
    sqlite: SqlitePool,
    dirty_changes: Changes,
    current_commit: CurrentCommit,
}

impl Storage {
    /// Creates a new [`Storage`] persisted by the given SQLite database.
    /// The SQLite database may or may not be an empty database. If it's empty,
    /// then it will be initialized with the appropriate schema.
    pub async fn new(sqlite_uri: &str) -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(sqlite_uri).await?;
        sqlx::query(SQL_SCHEMA).execute(&sqlite).await?;

        let current_commit_id = max_commit_id(&sqlite).await?;

        let gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            current_commit: CurrentCommit {
                id: current_commit_id.unwrap_or(INITIAL_COMMIT_ID),
                changes: HashMap::new(),
                changes_xor_fingerprint: FINGERPRINT_ONES,
            },
        };

        if current_commit_id.is_none() {
            gs.insert_commit(INITIAL_COMMIT_ID, FINGERPRINT_ZEROS)
                .await?;
        }

        Ok(gs)
    }

    /// Creates a new, empty [`Storage`] with no persisted state at all. All
    /// state will be kept in an in-memory SQLite instance.
    pub async fn in_memory() -> Result<Self> {
        Self::new(":memory:").await
    }

    /// Fetches the value associated with the Blake3 hash of `key`. See
    /// [`Storage::get_by_hash`] for more information.
    pub async fn get(&self, key: &[u8], commit: Option<&Fingerprint>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, commit).await
    }

    /// Fetches the value associated with a Blake3 `hash`. If `commit`
    /// is `None`, the most recent value will be returned; otherwise, only the
    /// values present at the time of `commit` would be considered.
    pub async fn get_by_hash(
        &self,
        hash: &Fingerprint,
        commit: Option<&Fingerprint>,
    ) -> Result<Option<Vec<u8>>> {
        // We are given an explicit [`Fingerprint`], so we must add that condition.
        if let Some(commit) = commit {
            let value_opt: Option<(Vec<u8>,)> = sqlx::query_as(
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
            )
            .bind(&commit[..])
            .bind(&hash[..])
            .fetch_optional(&self.sqlite)
            .await?;
            Ok(value_opt.map(|x| x.0))
        } else if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.clone()))
        } else if let Some(value) = self.current_commit.changes.get(hash) {
            Ok(Some(value.clone()))
        } else {
            // Let's fetch the most recent record and see if we find anything.
            let value: Option<(Vec<u8>,)> = sqlx::query_as(
                r#"
                    SELECT "value"
                    FROM "values"
                    WHERE "key_hash" = ?1
                    ORDER BY "id" DESC
                    LIMIT 1
                "#,
            )
            .bind(&hash[..])
            .fetch_optional(&self.sqlite)
            .await?;

            Ok(value.map(|x| x.0))
        }
    }

    /// Sets the `value` associated with the Blake3 hash of `key`. See
    /// [`Storage::upsert_by_hash`] for more information.
    pub async fn upsert<V>(&mut self, key: &[u8], value: V)
    where
        V: Into<Vec<u8>>,
    {
        let hash = Blake3Hasher::hash(key);
        self.upsert_by_hash(hash, value).await;
    }

    /// Sets the `value` associated with a Blake3 `hash`. This change will be
    /// "dirty" until a [`Storage::checkout`].
    pub async fn upsert_by_hash<V>(&mut self, hash: Fingerprint, value: V)
    where
        V: Into<Vec<u8>>,
    {
        let value = value.into();
        self.dirty_changes.entry(hash).or_insert(value);
    }

    /// Prepares dirty changes to be commited via [`Storage::commit`]. After
    /// saving, changes are frozen and can't be removed from the current commit.
    pub async fn checkpoint(&mut self) {
        let dirty_changes = std::mem::take(&mut self.dirty_changes);

        let mut fingerprint = self.current_commit.changes_xor_fingerprint;

        for change in dirty_changes {
            xor_fingerprint(&mut fingerprint, &hash_key_value_pair(&change.0, &change.1));
            self.current_commit.changes.insert(change.0, change.1);
        }
    }

    /// Persists all changes to disk and returns the root fingerprint of the new
    /// commit. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`Storage::checkpoint`] before
    /// this call.
    pub async fn commit(&mut self) -> Result<Fingerprint> {
        if !self.dirty_changes.is_empty() {
            Err(StorageError::DirtyChanges)
        } else {
            let mut inserts = vec![];
            let commit_changes = std::mem::take(&mut self.current_commit.changes);

            for (key_hash, value) in commit_changes {
                inserts.push(
                    sqlx::query(
                        r#"
                            INSERT INTO "values" ("key_hash", "value", "commit_id")
                            VALUES (?1, ?2, ?3)
                        "#,
                    )
                    .bind(key_hash.to_vec())
                    .bind(value)
                    .bind(self.current_commit.id)
                    .execute(&self.sqlite),
                )
            }

            futures::future::try_join_all(inserts).await?;

            let new_fingerprint = self
                .update_commit_fingerprint(
                    self.current_commit.id,
                    &self.current_commit.changes_xor_fingerprint,
                )
                .await?;

            self.current_commit.changes_xor_fingerprint = FINGERPRINT_ONES;
            self.current_commit.id += 1;

            self.insert_commit(self.current_commit.id, new_fingerprint)
                .await?;

            Ok(new_fingerprint)
        }
    }

    /// Creates a new [`Fingerprint`]-ed commit with the given `id`.
    async fn insert_commit(&self, id: i64, fingerprint: Fingerprint) -> Result<()> {
        sqlx::query(
            r#"
                INSERT INTO "commits" ("id", "fingerprint")
                VALUES (?1, ?2)
            "#,
        )
        .bind(id)
        .bind(&fingerprint[..])
        .execute(&self.sqlite)
        .await?;
        Ok(())
    }

    /// Queries the current [`Fingerprint`] value of `commit_id`.
    async fn commit_fingerprint(&self, commit_id: i64) -> Result<Fingerprint> {
        let bytes: (Vec<u8>,) = sqlx::query_as(
            r#"
                SELECT "fingerprint"
                FROM "commits"
                WHERE "id" = ?1
            "#,
        )
        .bind(commit_id)
        .fetch_one(&self.sqlite)
        .await?;
        Ok(bytes.0.try_into().unwrap())
    }

    /// XOR's the [`Fingerprint`] of `commit_id` with `xor`.
    async fn update_commit_fingerprint(
        &self,
        commit_id: i64,
        xor: &Fingerprint,
    ) -> Result<Fingerprint> {
        let mut fingerprint: Fingerprint = self.commit_fingerprint(self.current_commit.id).await?;
        xor_fingerprint(&mut fingerprint, xor);

        sqlx::query(
            r#"
                UPDATE "commits"
                SET "fingerprint" = ?1
                WHERE "id" = ?2
            "#,
        )
        .bind(&fingerprint[..])
        .bind(commit_id)
        .execute(&self.sqlite)
        .await?;

        Ok(fingerprint)
    }

    /// Returns the [`Fingerprint`] of the last ever checkpoint; i.e.
    /// persisted changes without dirty and saved changes.
    pub async fn current(&self) -> Result<Fingerprint> {
        self.commit_fingerprint(self.current_commit.id).await
    }

    /// Erases all dirty changes and saved data from memory. Persisted data is
    /// left untouched.
    pub async fn rewind(&mut self, _commit: &Fingerprint) -> Result<()> {
        if self.dirty_changes.is_empty() {
            Ok(())
        } else {
            Err(StorageError::DirtyChanges)
        }
    }

    /// Erases all dirty changes from memory. Persisted and saved data are left
    /// untouched.
    pub fn rollback(&mut self) {
        self.dirty_changes.clear();
    }
}

async fn max_commit_id(pool: &SqlitePool) -> Result<Option<i64>> {
    let max_commit_id: Option<(i64,)> = sqlx::query_as(
        r#"
                SELECT "id"
                FROM "commits"
                ORDER BY "id" DESC
                LIMIT 1
            "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(max_commit_id.map(|x| x.0))
}

fn hash_key_value_pair(key_hash: &Fingerprint, value: &[u8]) -> Fingerprint {
    let mut hasher = Blake3Hasher::default();
    hasher.update(key_hash);
    hasher.update(value);
    hasher.finalize()
}

fn xor_fingerprint(sig_1: &mut Fingerprint, sig_2: &Fingerprint) {
    for (a, b) in sig_1.iter_mut().zip(sig_2) {
        *a ^= *b;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[quickcheck_async::tokio]
    async fn retrieval_after_dirty_changes(items: HashMap<String, String>) -> bool {
        let mut gs = Storage::in_memory().await.unwrap();

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
        let mut gs = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter().filter(|kv| !kv.0.is_empty()) {
            gs.upsert(key.as_bytes(), value.as_bytes()).await;
        }

        gs.checkpoint().await;

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
        let mut gs = Storage::in_memory().await.unwrap();

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
        let gs = Storage::in_memory().await.unwrap();
        matches!(gs.get(&key, None).await, Ok(None))
    }

    #[quickcheck_async::tokio]
    async fn rewind_succeeds_when_empty(bytes: Vec<u8>) -> bool {
        let commit = Blake3Hasher::hash(&bytes);
        let mut gs = Storage::in_memory().await.unwrap();

        gs.rewind(&commit).await.is_ok()
    }

    #[quickcheck_async::tokio]
    async fn rewind_fails_after_insert(bytes: Vec<u8>, key: Vec<u8>, value: Vec<u8>) -> bool {
        let commit = Blake3Hasher::hash(&bytes);
        let mut gs = Storage::in_memory().await.unwrap();

        gs.upsert(&key, value).await;
        gs.rewind(&commit).await.is_err()
    }
}
