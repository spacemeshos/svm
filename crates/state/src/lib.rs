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

/// Layer identifiers are unsigned 64-bit integers.
pub type LayerId = u64;

type Changes = HashMap<Fingerprint, Vec<u8>>;

const SQL_SCHEMA: &str = include_str!("resources/schema.sql");
const INITIAL_LAYER_ID: LayerId = 0;

// When initializing [`Fingerprint`]'s on the database, we zero all bits. In
// memory, however, the initial [`Fingerprint`] is always filled with 1's.
const FINGERPRINT_ZEROS: Fingerprint = [0; 32];
const FINGERPRINT_ONES: Fingerprint = [std::u8::MAX; 32];

struct CurrentLayer {
    id: LayerId,
    changes: Changes,
    changes_xor_fingerprint: Fingerprint,
}

/// A SQLite-backed key-value store that supports root fingerprinting and
/// historical state queries.
pub struct Storage {
    sqlite: SqlitePool,
    dirty_changes: Changes,
    current_layer: CurrentLayer,
}

impl Storage {
    /// Creates a new [`Storage`] persisted by the given SQLite database.
    /// The SQLite database may or may not be an empty database. If it's empty,
    /// then it will be initialized with the appropriate schema.
    pub async fn new(sqlite_uri: &str) -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(sqlite_uri).await?;
        sqlx::query(SQL_SCHEMA).execute(&sqlite).await?;

        let current_layer_id = max_layer_id(&sqlite).await?;

        let gs = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            current_layer: CurrentLayer {
                id: current_layer_id.unwrap_or(INITIAL_LAYER_ID),
                changes: HashMap::new(),
                changes_xor_fingerprint: FINGERPRINT_ONES,
            },
        };

        if current_layer_id.is_none() {
            gs.insert_layer(INITIAL_LAYER_ID as i64, FINGERPRINT_ZEROS)
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
    pub async fn get(&self, key: &[u8], layer: Option<LayerId>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, layer).await
    }

    /// Fetches the value associated with a Blake3 `hash`. If `layer`
    /// is `None`, the most recent value will be returned; otherwise, only the
    /// values present at the time of `layer` would be considered.
    pub async fn get_by_hash(
        &self,
        hash: &Fingerprint,
        layer: Option<LayerId>,
    ) -> Result<Option<Vec<u8>>> {
        if let Some(layer) = layer {
            self.get_by_hash_historical(hash, layer).await
        } else if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.clone()))
        } else if let Some(value) = self.current_layer.changes.get(hash) {
            Ok(Some(value.clone()))
        } else {
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

    async fn get_by_hash_historical(
        &self,
        hash: &Fingerprint,
        layer: LayerId,
    ) -> Result<Option<Vec<u8>>> {
        let value_opt: Option<(Vec<u8>,)> = sqlx::query_as(
            r#"
                SELECT "value"
                FROM "values"
                WHERE
                    "layer_id" <= (
                        SELECT "id"
                        FROM "layers"
                        WHERE "layer_id" = ?1
                    )
                    AND
                    "key_hash" = ?2
                ORDER BY "id" DESC
                LIMIT 1
                "#,
        )
        .bind(layer as i64)
        .bind(&hash[..])
        .fetch_optional(&self.sqlite)
        .await?;
        Ok(value_opt.map(|x| x.0))
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
        self.dirty_changes.entry(hash).or_insert(value.into());
    }

    /// Prepares dirty changes to be layered via [`Storage::layer`]. After
    /// saving, changes are frozen and can't be removed from the current layer.
    pub async fn checkpoint(&mut self) {
        let mut fingerprint = self.current_layer.changes_xor_fingerprint;
        let dirty_changes = std::mem::take(&mut self.dirty_changes);

        for change in dirty_changes {
            let xor = hash_key_value_pair(&change.0, &change.1);
            xor_fingerprint(&mut fingerprint, &xor);

            self.current_layer.changes.insert(change.0, change.1);
        }
    }

    /// Persists all changes to disk and returns the root fingerprint of the new
    /// layer. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`Storage::checkpoint`] before
    /// this call.
    pub async fn commit(&mut self) -> Result<Fingerprint> {
        if !self.dirty_changes.is_empty() {
            return Err(StorageError::DirtyChanges);
        }

        // Note: SQLx 0.5 doesn't support bulk inserts. While tempting,
        // inserting one by one and `.await`-ing after every operation is
        // terribly slow. Rather, we store operation futures in a [`Vec`] and we
        // then use [`futures`] magic.
        let mut inserts = vec![];
        let layer_changes = std::mem::take(&mut self.current_layer.changes);

        for (key_hash, value) in layer_changes {
            inserts.push(
                sqlx::query(
                    r#"
                    INSERT INTO "values" ("key_hash", "value", "layer_id")
                    VALUES (?1, ?2, ?3)
                    "#,
                )
                .bind(key_hash.to_vec())
                .bind(value)
                .bind(self.current_layer.id as i64)
                .execute(&self.sqlite),
            )
        }

        futures::future::try_join_all(inserts).await?;

        let new_fingerprint = self
            .update_layer_fingerprint(
                self.current_layer.id as i64,
                &self.current_layer.changes_xor_fingerprint,
            )
            .await?;

        // Reset layer-specific information to default values.
        self.current_layer.changes_xor_fingerprint = FINGERPRINT_ONES;
        self.current_layer.id += 1;

        // Finally, create an empty layer for subsequent operations.
        self.insert_layer(self.current_layer.id as i64, new_fingerprint)
            .await?;

        Ok(new_fingerprint)
    }

    /// Creates a new [`Fingerprint`]-ed layer with the given `id`.
    async fn insert_layer(&self, id: i64, fingerprint: Fingerprint) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO "layers" ("id", "fingerprint")
            VALUES (?1, ?2)
            "#,
        )
        .bind(id)
        .bind(&fingerprint[..])
        .execute(&self.sqlite)
        .await?;
        Ok(())
    }

    /// Queries the current [`Fingerprint`] value of `layer_id`.
    async fn layer_fingerprint(&self, layer: LayerId) -> Result<Fingerprint> {
        let bytes: (Vec<u8>,) = sqlx::query_as(
            r#"
            SELECT "fingerprint"
            FROM "layers"
            WHERE "id" = ?1
            "#,
        )
        .bind(layer as i64)
        .fetch_one(&self.sqlite)
        .await?;
        Ok(bytes.0.try_into().unwrap())
    }

    /// XOR's the [`Fingerprint`] of `layer_id` with `xor`.
    async fn update_layer_fingerprint(
        &self,
        layer_id: i64,
        xor: &Fingerprint,
    ) -> Result<Fingerprint> {
        let mut fingerprint: Fingerprint = self.layer_fingerprint(self.current_layer.id).await?;
        xor_fingerprint(&mut fingerprint, xor);

        sqlx::query(
            r#"
            UPDATE "layers"
            SET "fingerprint" = ?1
            WHERE "id" = ?2
            "#,
        )
        .bind(&fingerprint[..])
        .bind(layer_id)
        .execute(&self.sqlite)
        .await?;

        Ok(fingerprint)
    }

    /// Returns the [`Fingerprint`] of the last ever checkpoint; i.e.
    /// persisted changes without dirty and saved changes.
    pub async fn current(&self) -> Result<Fingerprint> {
        self.layer_fingerprint(self.current_layer.id).await
    }

    /// Completely deletes all layers after `layer` from the SQLite store.
    /// Returns a [`StorageError::Changes`] or [`StorageError::DirtyChanges`] in
    /// case there's in-memory changes that haven't been persisted yet.
    pub async fn erase_history(&mut self, layer: &Fingerprint) -> Result<()> {
        if !self.dirty_changes.is_empty() {
            Err(StorageError::DirtyChanges)
        } else if !self.current_layer.changes.is_empty() {
            Err(StorageError::Changes)
        } else {
            sqlx::query(
                r#"
                DELETE FROM "layers"
                WHERE "layer_id" > (
                    SELECT "id"
                    FROM "layers"
                    WHERE "fingerprint" = ?1
                )
                "#,
            )
            .bind(&layer[..])
            .execute(&self.sqlite)
            .await?;
            Ok(())
        }
    }

    /// Erases all saved data from memory. Persisted data is
    /// left untouched. It returns a [`StorageError::DirtyChanges`] in case
    /// there's any dirty changes, i.e. you must call [`Storage::rollback`]
    /// beforehand.
    pub async fn rewind(&mut self, _layer: &Fingerprint) -> Result<()> {
        if self.dirty_changes.is_empty() {
            self.current_layer.changes.clear();
            self.current_layer.changes_xor_fingerprint = FINGERPRINT_ONES;
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

async fn max_layer_id(pool: &SqlitePool) -> Result<Option<LayerId>> {
    let max_layer_id: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT "id"
        FROM "layers"
        ORDER BY "id" DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await?;

    Ok(max_layer_id.map(|x| x.0.try_into().expect("Negative layer ID!")))
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
        gs.checkpoint().await;
        let layer_1 = gs.commit().await.unwrap();

        gs.upsert(b"foo", "spam").await;
        gs.checkpoint().await;
        let layer_2 = gs.commit().await.unwrap();

        gs.upsert(b"foo", "bar").await;
        gs.checkpoint().await;
        let layer_3 = gs.commit().await.unwrap();

        layer_1 != layer_2 && layer_1 == layer_3
    }

    #[quickcheck_async::tokio]
    async fn get_fails_on_empty_global_state(key: Vec<u8>) -> bool {
        let gs = Storage::in_memory().await.unwrap();
        matches!(gs.get(&key, None).await, Ok(None))
    }

    #[quickcheck_async::tokio]
    async fn rewind_succeeds_when_empty(bytes: Vec<u8>) -> bool {
        let layer = Blake3Hasher::hash(&bytes);
        let mut gs = Storage::in_memory().await.unwrap();

        gs.rewind(&layer).await.is_ok()
    }

    #[quickcheck_async::tokio]
    async fn rewind_fails_after_insert(bytes: Vec<u8>, key: Vec<u8>, value: Vec<u8>) -> bool {
        let layer = Blake3Hasher::hash(&bytes);
        let mut gs = Storage::in_memory().await.unwrap();

        gs.upsert(&key, value).await;
        gs.rewind(&layer).await.is_err()
    }
}
