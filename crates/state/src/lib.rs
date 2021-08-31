//! Implementation of Global State for the Spacemesh Virtual Machine.

mod account_data;
mod error;
mod global_state;
mod storage;

pub use error::{StorageError, StorageResult};
pub use global_state::GlobalState;
