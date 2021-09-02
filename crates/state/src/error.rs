use std::fmt::Debug;

use svm_types::State;

/// An alias for [`Result`](std::result::Result)'s with [`StorageError`].
pub type StorageResult<T> = std::result::Result<T, StorageError>;

/// A sum type for all error conditions that can arise in this crate.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// The storage layer contains some dirty changes that must be either saved
    /// or rollbacked before attempting such operation.
    #[error("Please checkout dirty changes or rollback to avoid data loss.")]
    DirtyChanges,

    /// Two checkpoints have resulted in key collision, which prevents unordered
    /// transaction replaying.
    #[error("Key collision from two different checkpoints.")]
    KeyCollision {
        /// They Blake3 hash of the key that caused this collision.
        key_hash: [u8; 32],
    },

    /// Illegal data found in the database.
    #[error("Illegal data found in the database")]
    IllegalData { key_hash: [u8; 32] },

    /// Expected an item in the database, but wasn't found.
    #[error("Expected an item in the database, but wasn't found.")]
    NotFound { key_hash: [u8; 32] },

    /// A SQLite error happened.
    #[error("SQLite error.")]
    Sqlite(#[from] sqlx::Error),
}
