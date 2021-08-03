use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum GlobalStateError {
    #[error("Invalid item.")]
    InvalidItem,
    #[error("Please commit dirty changes or rollback to avoid data loss.")]
    DirtyChanges,
    #[error("Hash collision. Make sure that you're not using the hash of a known item.")]
    Collision,
    #[error(
        "Possible self-reference at the database layer. We couldn't reach the leaf fast enough."
    )]
    Cyclic,
    #[error("SQLite error.")]
    Sqlite(#[from] sqlx::Error),
}
