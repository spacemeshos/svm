//! Implementation of Global State for the Spacemesh Virtual Machine.

mod error;
mod gs;
mod storage;

pub use error::{StorageError, StorageResult};
pub use gs::GlobalState;
