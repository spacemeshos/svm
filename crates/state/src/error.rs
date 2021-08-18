use std::fmt::Debug;

/// An alias for [`Result`](std::result::Result)'s with [`StorageError`].
pub type Result<T> = std::result::Result<T, StorageError>;

/// A sum type for all error conditions that can arise in this crate.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// The storage layer contains some dirty changes that must be either saved
    /// or rollbacked before attempting such operation.
    #[error("Please checkout dirty changes or rollback to avoid data loss.")]
    DirtyChanges,

    /// A SQLite error happened.
    #[error("SQLite error.")]
    Sqlite(#[from] sqlx::Error),
}
