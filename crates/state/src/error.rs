use std::fmt::Debug;

#[derive(Debug, Clone, thiserror::Error)]
pub enum GlobalStateError<E>
where
    E: Debug,
{
    #[error("Invalid item")]
    InvalidItem,
    #[error("Hash collision. Make sure that you're not using the hash of a known item.")]
    Collision,
    #[error(
        "Possible self-reference at the database layer. We couldn't reach the leaf fast enough."
    )]
    Cyclic,
    #[error("Backend I/O error: {0:?}.")]
    Backend(E),
}
