//! Implementation of Global State for the Spacemesh Virtual Machine.

#![warn(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]

mod account_data;
mod error;
mod global_state;
mod storage;

pub use error::{StorageError, StorageResult};
pub use global_state::GlobalState;
