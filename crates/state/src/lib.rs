//! Global State implementation for the SVM.

#![deny(missing_docs)]
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

// When initializing [`Fingerprint`]'s on the database, we zero all bits. In
// memory, however, the initial [`Fingerprint`] is always filled with 1's (by
// XOR-ing them together, we still get 0).
const FINGERPRINT_ZEROS: Fingerprint = [0; 32];
const FINGERPRINT_ONES: Fingerprint = [std::u8::MAX; 32];

const SQL_SCHEMA: &str = include_str!("resources/schema.sql");
const INITIAL_LAYER_ID: LayerId = 0;

struct CurrentLayer {
    id: LayerId,
    changes: Changes,
    changes_xor_fingerprint: Fingerprint,
}

/// A SQLite-backed key-value store that supports root fingerprinting and
/// historical state queries. This is the data structure that ultimately backs
/// the higher-level Global State APIs.
///
/// Please note that **all** operations might trigger a
/// [`StorageError::Sqlite`], unless otherwise specified.
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

        let storage = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            current_layer: CurrentLayer {
                id: current_layer_id.unwrap_or(INITIAL_LAYER_ID + 1),
                changes: HashMap::new(),
                changes_xor_fingerprint: FINGERPRINT_ONES,
            },
        };

        if current_layer_id.is_none() {
            storage
                .insert_layer(INITIAL_LAYER_ID as i64, FINGERPRINT_ZEROS, true)
                .await?;
            storage
                .insert_layer(INITIAL_LAYER_ID as i64 + 1, FINGERPRINT_ZEROS, false)
                .await?;
        }

        Ok(storage)
    }

    /// Creates a new, empty [`Storage`] with no persisted state at all. All
    /// state will be kept in an in-memory SQLite instance.
    pub async fn in_memory() -> Result<Self> {
        Self::new(":memory:").await
    }

    /// Fetches the value associated with the Blake3 hash of `key`. See
    /// [`Storage::get_by_hash`] for more information.
    pub async fn get(&self, key: &[u8], layer_id: Option<LayerId>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, layer_id).await
    }

    /// Fetches the value associated with a Blake3 `hash`. If `layer`
    /// is `None`, the most recent value will be returned; otherwise, only the
    /// values present at the time of `layer` would be considered.
    pub async fn get_by_hash(
        &self,
        hash: &Fingerprint,
        layer_id: Option<LayerId>,
    ) -> Result<Option<Vec<u8>>> {
        if let Some(layer) = layer_id {
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
        layer_id: LayerId,
    ) -> Result<Option<Vec<u8>>> {
        let value_opt: Option<(Vec<u8>,)> = sqlx::query_as(
            r#"
                SELECT "values.value"
                FROM "values"
                INNER JOIN "layers" ON
                    "layers.id" <= ?1
                    AND
                    "layers.ready" = 1
                    AND
                    "values.key_hash" = ?2
                ORDER BY "id" DESC
                LIMIT 1
                "#,
        )
        .bind(layer_id as i64)
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
    /// "dirty" until a [`Storage::checkpoint`].
    pub async fn upsert_by_hash<V>(&mut self, hash: Fingerprint, value: V)
    where
        V: Into<Vec<u8>>,
    {
        self.dirty_changes.entry(hash).or_insert(value.into());
    }

    /// Saves dirty changes in preparation of [`Storage::commit`]. After
    /// saving, changes are frozen and can't be removed from the current layer.
    ///
    /// This might return a [`StorageError::KeyCollision`] depending on the
    /// content of the dirty changes, so beware.
    pub async fn checkpoint(&mut self) -> Result<()> {
        let mut fingerprint = self.current_layer.changes_xor_fingerprint;
        let dirty_changes = std::mem::take(&mut self.dirty_changes);

        for change in dirty_changes {
            let xor = hash_key_value_pair(&change.0, &change.1);
            xor_fingerprint(&mut fingerprint, &xor);

            if self
                .current_layer
                .changes
                .insert(change.0, change.1)
                .is_some()
            {
                return Err(StorageError::KeyCollision { key_hash: change.0 });
            }
        }

        Ok(())
    }

    /// Persists all changes to disk and returns the root fingerprint of the new
    /// layer. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`Storage::checkpoint`] before
    /// this call.
    pub async fn commit(&mut self) -> Result<(LayerId, Fingerprint)> {
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
            .release_layer(
                self.current_layer.id as i64,
                &self.current_layer.changes_xor_fingerprint,
            )
            .await?;

        // Reset layer-specific information to default values.
        self.current_layer.changes_xor_fingerprint = FINGERPRINT_ONES;
        let layer_id = self.current_layer.id;
        self.current_layer.id += 1;

        // Finally, create an empty layer for subsequent operations.
        self.insert_layer(self.current_layer.id as i64, new_fingerprint, false)
            .await?;

        Ok((layer_id, new_fingerprint))
    }

    /// Creates a new [`Fingerprint`]-ed layer with the given `id`.
    async fn insert_layer(&self, id: i64, fingerprint: Fingerprint, ready: bool) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO "layers" ("id", "fingerprint", "ready")
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(id)
        .bind(&fingerprint[..])
        .bind(if ready { 1 } else { 0 })
        .execute(&self.sqlite)
        .await?;
        Ok(())
    }

    /// Queries the current [`Fingerprint`] value of `layer_id`.
    async fn layer_fingerprint(&self, layer_id: LayerId, ready: bool) -> Result<Fingerprint> {
        let bytes: (Vec<u8>,) = sqlx::query_as(
            r#"
            SELECT "fingerprint"
            FROM "layers"
            WHERE "id" = ?1 AND "ready" = ?2
            "#,
        )
        .bind(layer_id as i64)
        .bind(if ready { 1 } else { 0 })
        .fetch_one(&self.sqlite)
        .await?;
        Ok(bytes.0.try_into().unwrap())
    }

    /// XOR's the [`Fingerprint`] of `layer_id` with `xor` and marks it as
    /// "ready", i.e. becomes part of history and can be queried.
    async fn release_layer(&self, layer_id: i64, xor: &Fingerprint) -> Result<Fingerprint> {
        let mut fingerprint: Fingerprint =
            self.layer_fingerprint(self.current_layer.id, false).await?;
        xor_fingerprint(&mut fingerprint, xor);

        sqlx::query(
            r#"
            UPDATE "layers"
            SET "fingerprint" = ?1, "ready" = 1
            WHERE "id" = ?2
            "#,
        )
        .bind(&fingerprint[..])
        .bind(layer_id)
        .execute(&self.sqlite)
        .await?;

        Ok(fingerprint)
    }

    /// Returns the [`Fingerprint`] of the last ever committed layer; i.e.
    /// persisted changes without dirty and saved changes.
    pub async fn current(&self) -> Result<Fingerprint> {
        self.layer_fingerprint(self.current_layer.id, true).await
    }

    /// Erases all saved data from memory and completely deletes all layers
    /// after and excluding `layer_id` from the SQLite store. Persisted data is
    /// left untouched. It returns a [`StorageError::DirtyChanges`] in case
    /// there's any dirty changes, i.e. you must call [`Storage::rollback`]
    /// beforehand.
    ///
    /// # Panics
    ///
    /// Panics if the given `layer_id` is invalid, i.e. nonexistant.
    pub async fn rewind(&mut self, layer_id: LayerId) -> Result<()> {
        // `self.current_layer_id` is not a "real" layer yet, so it's not a
        // valid target.
        assert!(layer_id < self.current_layer.id);

        if self.dirty_changes.is_empty() {
            sqlx::query(
                r#"
                DELETE FROM "layers"
                WHERE "id" > ?1
                "#,
            )
            .bind(layer_id as i64)
            .execute(&self.sqlite)
            .await?;

            // We must now bring `self.current_layer` in a good state.
            self.current_layer.id = layer_id + 1;
            self.current_layer.changes.clear();
            self.current_layer.changes_xor_fingerprint = FINGERPRINT_ONES;

            // Recreate last layer information in SQLite.
            let fingerprint = self.layer_fingerprint(layer_id, true).await?;
            self.insert_layer(self.current_layer.id as i64, fingerprint, false)
                .await?;

            Ok(())
        } else {
            Err(StorageError::DirtyChanges)
        }
    }

    /// Erases all dirty changes from memory. Persisted and saved data are left
    /// untouched.
    pub async fn rollback(&mut self) -> Result<()> {
        self.dirty_changes.clear();
        Ok(())
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
    async fn get_after_dirty_changes(items: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter() {
            storage.upsert(&key[..], &value[..]).await;
        }

        for (key, value) in items {
            let stored_value = storage.get(&key, None).await.unwrap().unwrap();
            if stored_value != value {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn get_after_checkpoint(items: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter() {
            storage.upsert(&key[..], &value[..]).await;
        }

        storage.checkpoint().await.unwrap();

        for (key, value) in items {
            let stored_value = storage.get(&key, None).await.unwrap().unwrap();
            if stored_value != value {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn get_after_commit(items: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter() {
            storage.upsert(&key[..], &value[..]).await;
        }

        storage.checkpoint().await.unwrap();
        storage.commit().await.unwrap();

        for (key, value) in items {
            let stored_value = storage.get(&key, None).await.unwrap().unwrap();
            if stored_value != value {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn get_historical_after_dirty_changes(items: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter() {
            storage.upsert(&key[..], &value[..]).await;
        }

        for key in items.keys() {
            let stored_value = storage.get(&key, Some(INITIAL_LAYER_ID)).await.unwrap();
            if stored_value.is_some() {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn get_historical_after_checkpoint(items: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter() {
            storage.upsert(&key[..], &value[..]).await;
        }

        storage.checkpoint().await.unwrap();

        for key in items.keys() {
            let stored_value = storage.get(&key, Some(INITIAL_LAYER_ID)).await.unwrap();
            if stored_value.is_some() {
                return false;
            }
        }

        true
    }

    #[quickcheck_async::tokio]
    async fn consistent_layer_information_after_commits() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();
        let layer_1 = storage.commit().await.unwrap();

        storage.upsert(b"foo", "spam").await;
        storage.checkpoint().await.unwrap();
        let layer_2 = storage.commit().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();
        let layer_3 = storage.commit().await.unwrap();

        // Check layer ID ordering
        layer_1.0 < layer_2.0
            && layer_2.0 < layer_3.0
        // Check layer fingerprinting
            && layer_1.1 != layer_2.1
            && layer_1.1 == layer_3.1
    }

    #[quickcheck_async::tokio]
    async fn get_immediately_after_new(key: Vec<u8>) -> bool {
        let storage = Storage::in_memory().await.unwrap();
        matches!(storage.get(&key, None).await, Ok(None))
    }

    #[quickcheck_async::tokio]
    async fn rollback_succeeds_when_empty() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.rollback().await.is_ok()
    }

    #[tokio::test]
    #[should_panic]
    async fn rewind_panics_without_commits() {
        let mut storage = Storage::in_memory().await.unwrap();
        storage.rewind(INITIAL_LAYER_ID).await.unwrap();
    }

    #[quickcheck_async::tokio]
    async fn rewind_succeeds_after_commit() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();
        let layer_id = storage.commit().await.unwrap().0;

        storage.rewind(layer_id).await.is_ok()
    }

    #[quickcheck_async::tokio]
    async fn rewind_fails_with_dirty_changes() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();
        let layer_id = storage.commit().await.unwrap().0;

        storage.upsert(b"spam", "foo").await;

        storage.rewind(layer_id).await.is_err()
    }

    #[quickcheck_async::tokio]
    async fn rewind_succeeds_with_saved_changes() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();
        let layer_id = storage.commit().await.unwrap().0;

        storage.upsert(b"spam", "foo").await;
        storage.checkpoint().await.unwrap();

        storage.rewind(layer_id).await.is_ok()
    }

    #[quickcheck_async::tokio]
    async fn rewind_effectively_deletes_data(key: Vec<u8>, value: Vec<u8>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        let layer_id = storage.commit().await.unwrap().0;

        storage.upsert(&key, value).await;
        storage.checkpoint().await.unwrap();
        storage.rewind(layer_id).await.unwrap();

        matches!(storage.get(&key, None).await, Ok(None))
    }

    #[quickcheck_async::tokio]
    async fn rewind_then_overwrite_keys_then_get() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.upsert(b"foo", "1").await;
        storage.upsert(b"bar", "2").await;
        storage.checkpoint().await.unwrap();

        storage.upsert(b"xyz", "1337").await;
        storage.checkpoint().await.unwrap();

        storage.upsert(b"spam", "42").await;
        storage.upsert(b"super-spam", "100").await;
        storage.rollback().await.unwrap();

        storage.upsert(b"super-spam", "50").await;
        storage.checkpoint().await.unwrap();

        storage.commit().await.unwrap();

        storage.get(b"foo", None).await.unwrap().unwrap() == b"1"
            && storage.get(b"bar", None).await.unwrap().unwrap() == b"2"
            && storage.get(b"xyz", None).await.unwrap().unwrap() == b"1337"
            && storage.get(b"spam", None).await.unwrap().is_none()
            && storage.get(b"super-spam", None).await.unwrap().unwrap() == b"50"
    }

    #[quickcheck_async::tokio]
    async fn checkpoint_ordering_doesnt_change_fingerprint() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        let layer_id = storage.commit().await.unwrap().0;

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();

        storage.upsert(b"bar", "foo").await;
        storage.checkpoint().await.unwrap();

        let fingeprint_0 = storage.commit().await.unwrap().1;

        storage.rewind(layer_id).await.unwrap();

        storage.upsert(b"bar", "foo").await;
        storage.checkpoint().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();

        let fingerprint_1 = storage.commit().await.unwrap().1;
        fingeprint_0 == fingerprint_1
    }

    #[quickcheck_async::tokio]
    async fn checkpoint_collision_detection() -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        storage.upsert(b"foo", "bar").await;
        storage.checkpoint().await.unwrap();

        storage.upsert(b"foo", "spam").await;
        matches!(
            storage.checkpoint().await,
            Err(StorageError::KeyCollision { key_hash: _ })
        )
    }
}
