//! Storage data structure that back the Spacemesh Global State.
//!
//! At the time of writing, [`Storage`] is a "LinkedHashX", as explained in
//! https://eprint.iacr.org/2021/773.pdf, pg. 5-6. This enables fast root
//! fingerprinting and rewinding and we will later transition to Erigon-like
//! Merklelization walks.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

use sqlx::SqlitePool;

use std::collections::HashMap;
use std::convert::TryInto;

use svm_hash::{Blake3Hasher, Hasher};
use svm_types::Layer;

pub use crate::error::{StorageError, StorageResult as Result};

/// Hashes as well as root fingerprints are 256 bits long.
pub type Fingerprint = [u8; 32];

type Changes = HashMap<Fingerprint, Vec<u8>>;

// When initializing [`Fingerprint`]'s on the database, we zero all bits. In
// memory, however, the initial [`Fingerprint`] is always filled with 1's (by
// XOR-ing them together, we still get 0).
const FINGERPRINT_ZEROS: Fingerprint = [0; 32];
const FINGERPRINT_ONES: Fingerprint = [std::u8::MAX; 32];

const SQL_SCHEMA: &str = include_str!("resources/schema.sql");
const INITIAL_LAYER_ID: Layer = Layer(0);

#[derive(Debug)]
struct NextLayer {
    id: Layer,
    changes: Changes,
    changes_xor_fingerprint: Fingerprint,
}

/// A SQLite-backed key-value store that supports root fingerprinting and
/// historical state queries. This is the data structure that ultimately backs
/// the higher-level Global State APIs.
///
/// Please note that **all** operations might trigger a
/// [`StorageError::Sqlite`], unless otherwise specified.
#[derive(Debug)]
pub struct Storage {
    sqlite: SqlitePool,
    dirty_changes: Changes,
    next_layer: NextLayer,
}

impl Storage {
    /// Creates a new [`Storage`] persisted by the given SQLite database.
    /// The SQLite database may or may not be an empty database. If it's empty,
    /// then it will be initialized with the appropriate schema.
    pub async fn new(sqlite_uri: &str) -> Result<Self> {
        let sqlite = sqlx::SqlitePool::connect(sqlite_uri).await?;
        sqlx::query(SQL_SCHEMA).execute(&sqlite).await?;

        let next_layer_id = max_layer_id(&sqlite).await?;

        let storage = Self {
            sqlite,
            dirty_changes: HashMap::new(),
            next_layer: NextLayer {
                id: next_layer_id.unwrap_or(Layer(INITIAL_LAYER_ID.0 + 1)),
                changes: HashMap::new(),
                changes_xor_fingerprint: FINGERPRINT_ONES,
            },
        };

        if next_layer_id.is_none() {
            storage
                .insert_layer(INITIAL_LAYER_ID, FINGERPRINT_ZEROS, true)
                .await?;
        }

        storage.delete_bad_layers().await?;

        Ok(storage)
    }

    async fn delete_bad_layers(&self) -> Result<u64> {
        let count_rows: u64 = sqlx::query(
            r#"
            DELETE FROM "values"
            WHERE "layer_id" IN (
                SELECT "id"
                FROM "layers"
                WHERE "ready" = 0
            )
            "#,
        )
        .execute(&self.sqlite)
        .await?
        .rows_affected();

        Ok(count_rows)
    }

    /// Creates a new, empty [`Storage`] with no persisted state at all. All
    /// state will be kept in an in-memory SQLite instance.
    #[cfg(test)]
    async fn in_memory() -> Result<Self> {
        Self::new(":memory:").await
    }

    /// Returns the [`Layer`] and [`Fingerprint`] of the last ever committed
    /// layer; i.e. persisted changes without dirty and saved changes.
    pub async fn last_layer(&self) -> Result<(Layer, Fingerprint)> {
        assert!(self.next_layer.id > INITIAL_LAYER_ID);

        let layer_id = Layer(self.next_layer.id.0 - 1);
        let fingerprint = self.layer_fingerprint(layer_id, true).await?;

        Ok((layer_id, fingerprint))
    }

    fn assert_layer_id_is_ready(&self, layer_id: Layer) {
        assert!(layer_id < self.next_layer.id);
    }

    /// Fetches the value associated with the Blake3 hash of `key`. See
    /// [`Storage::get_by_hash`] for more information.
    ///
    /// # Panics
    ///
    /// Panics if `layer_id` is invalid.
    pub async fn get(&self, key: &[u8], layer_id: Option<Layer>) -> Result<Option<Vec<u8>>> {
        let hash = Blake3Hasher::hash(key);
        self.get_by_hash(&hash, layer_id).await
    }

    /// Fetches the value associated with a Blake3 `hash`. If `layer`
    /// is `None`, the most recent value will be returned; otherwise, only the
    /// values present at the time of `layer` would be considered.
    ///
    /// # Panics
    ///
    /// Panics if `layer_id` is invalid.
    pub async fn get_by_hash(
        &self,
        hash: &Fingerprint,
        layer_id: Option<Layer>,
    ) -> Result<Option<Vec<u8>>> {
        if let Some(layer_id) = layer_id {
            self.assert_layer_id_is_ready(layer_id);
            self.get_by_hash_historical(hash, layer_id).await
        } else if let Some(value) = self.dirty_changes.get(hash) {
            Ok(Some(value.clone()))
        } else if let Some(value) = self.next_layer.changes.get(hash) {
            Ok(Some(value.clone()))
        } else {
            let value: Option<(Vec<u8>,)> = sqlx::query_as(
                r#"
                SELECT "value"
                FROM "values"
                INNER JOIN "layers" ON
                    "ready" = 1
                    AND
                    "key_hash" = ?1
                ORDER BY "layer_id" DESC
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
        layer_id: Layer,
    ) -> Result<Option<Vec<u8>>> {
        let value_opt: Option<(Vec<u8>,)> = sqlx::query_as(
            r#"
            SELECT "value"
            FROM "values"
            INNER JOIN "layers" ON
                "layer_id" <= ?1
                AND
                "ready" = 1
                AND
                "key_hash" = ?2
            ORDER BY "layer_id" DESC
            LIMIT 1
            "#,
        )
        .bind(layer_id.0 as i64)
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
        let mut fingerprint = self.next_layer.changes_xor_fingerprint;
        let dirty_changes = std::mem::take(&mut self.dirty_changes);

        for change in dirty_changes {
            let xor = hash_key_value_pair(&change.0, &change.1);
            xor_fingerprint(&mut fingerprint, &xor);

            if self.next_layer.changes.insert(change.0, change.1).is_some() {
                return Err(StorageError::KeyCollision { key_hash: change.0 });
            }
        }

        Ok(())
    }

    /// Persists all changes to disk and returns the root fingerprint of the new
    /// layer. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`Storage::checkpoint`] before
    /// this call.
    pub async fn commit(&mut self) -> Result<(Layer, Fingerprint)> {
        if !self.dirty_changes.is_empty() {
            return Err(StorageError::DirtyChanges);
        }

        let layer_id = self.next_layer.id;

        // Note: SQLx 0.5 doesn't support bulk inserts. While tempting,
        // inserting one by one and `.await`-ing after every operation is
        // terribly slow. Rather, we store operation futures in a [`Vec`] and we
        // then use [`futures`] magic.
        let mut inserts = vec![];
        let layer_changes = std::mem::take(&mut self.next_layer.changes);

        let mut fingerprint = self.layer_fingerprint(Layer(layer_id.0 - 1), true).await?;
        xor_fingerprint(&mut fingerprint, &self.next_layer.changes_xor_fingerprint);

        self.insert_layer(layer_id, fingerprint, false).await?;

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
                .bind(layer_id.0 as i64)
                .execute(&self.sqlite),
            )
        }

        futures::future::try_join_all(inserts).await?;

        sqlx::query(
            r#"
            UPDATE "layers"
            SET "fingerprint" = ?1, "ready" = 1
            WHERE "id" = ?2 AND "ready" = 0
            "#,
        )
        .bind(&fingerprint[..])
        .bind(layer_id.0 as i64)
        .execute(&self.sqlite)
        .await?;

        // Reset layer-specific information to default values.
        self.next_layer.changes_xor_fingerprint = FINGERPRINT_ONES;
        self.next_layer.id.0 += 1;

        Ok((layer_id, fingerprint))
    }

    /// Creates a new [`Fingerprint`]-ed layer with the given `id`.
    async fn insert_layer(&self, id: Layer, fingerprint: Fingerprint, ready: bool) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO "layers" ("id", "fingerprint", "ready")
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(id.0 as i64)
        .bind(&fingerprint[..])
        .bind(if ready { 1 } else { 0 })
        .execute(&self.sqlite)
        .await?;
        Ok(())
    }

    /// Queries the current [`Fingerprint`] value of `layer_id`.
    async fn layer_fingerprint(&self, layer_id: Layer, ready: bool) -> Result<Fingerprint> {
        let bytes: (Vec<u8>,) = sqlx::query_as(
            r#"
            SELECT "fingerprint"
            FROM "layers"
            WHERE "id" = ?1 AND "ready" = ?2
            "#,
        )
        .bind(layer_id.0 as i64)
        .bind(if ready { 1 } else { 0 })
        .fetch_one(&self.sqlite)
        .await?;
        Ok(bytes.0.try_into().unwrap())
    }

    /// Erases all saved data from memory and completely deletes all layers
    /// after and excluding `layer_id` from the SQLite store. Persisted data is
    /// left untouched. It returns a [`StorageError::DirtyChanges`] in case
    /// there's any dirty changes, i.e. you must call [`Storage::rollback`]
    /// beforehand.
    ///
    /// # Panics
    ///
    /// Panics if `layer_id` is invalid.
    pub async fn rewind(&mut self, layer_id: Layer) -> Result<()> {
        self.assert_layer_id_is_ready(layer_id);

        if !self.dirty_changes.is_empty() {
            return Err(StorageError::DirtyChanges);
        }

        sqlx::query(
            r#"
            DELETE FROM "layers"
            WHERE "id" > ?1
            "#,
        )
        .bind(layer_id.0 as i64)
        .execute(&self.sqlite)
        .await?;

        // We must now bring `self.next_layer` in a good state.
        self.next_layer.id.0 = layer_id.0 + 1;
        self.next_layer.changes.clear();
        self.next_layer.changes_xor_fingerprint = FINGERPRINT_ONES;

        Ok(())
    }

    /// Erases all dirty changes from memory. Persisted and saved data are left
    /// untouched.
    pub async fn rollback(&mut self) -> Result<()> {
        self.dirty_changes.clear();
        Ok(())
    }
}

async fn max_layer_id(pool: &SqlitePool) -> Result<Option<Layer>> {
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

    Ok(max_layer_id.map(|x| Layer(x.0.try_into().expect("Negative layer ID!"))))
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
    async fn get_and_get_historical_might_be_the_same(items: HashMap<Vec<u8>, Vec<u8>>) -> bool {
        let mut storage = Storage::in_memory().await.unwrap();

        for (key, value) in items.iter() {
            storage.upsert(&key[..], &value[..]).await;
        }

        storage.checkpoint().await.unwrap();
        let layer_id = storage.commit().await.unwrap().0;

        for key in items.keys() {
            let val_1 = storage.get(&key, Some(layer_id)).await.unwrap();
            let val_2 = storage.get(&key, None).await.unwrap();
            if val_1 != val_2 || val_1.is_none() {
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

    #[tokio::test(flavor = "multi_thread")]
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
