use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Invalid item.")]
    InvalidItem,

    #[error("Please checkout dirty changes or rollback to avoid data loss.")]
    DirtyChanges,

    #[error("You must rewind before erasing commit data.")]
    SavedChanges,

    #[error("Changing the same value multiple times within the same layer is ")]
    Collision,

    #[error("SQLite error.")]
    Sqlite(#[from] sqlx::Error),
}
