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

    /// Illegal data found in the database.
    #[error("Illegal data found in the database")]
    IllegalData {
        /// They Blake3 hash of the key that is associated with the illegal
        /// data.
        key_hash: State,
    },

    /// Expected an item in the database, but wasn't found.
    #[error("Expected an item in the database, but wasn't found.")]
    NotFound {
        /// They Blake3 hash of the key that doesn't have a value.
        key_hash: State,
    },

    /// A SQLite error happened.
    #[error("SQLite error.")]
    Sqlite(#[from] sqlx::Error),
}
