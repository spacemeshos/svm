use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Invalid item.")]
    InvalidItem,

    #[error("Please checkout dirty changes or rollback to avoid data loss.")]
    DirtyChanges,

    #[error("You must rewind before erasing commit data.")]
    Changes,

    #[error("Hash collision. Make sure that you're not using the hash of a known item.")]
    Collision,

    #[error(
        "Possible self-reference at the database layer. We couldn't reach the leaf fast enough."
    )]
    Cyclic,

    #[error("SQLite error.")]
    Sqlite(#[from] sqlx::Error),
}
