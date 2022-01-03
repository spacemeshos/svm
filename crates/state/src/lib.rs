//! Implementation of Global State for the Spacemesh Virtual Machine.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]

mod account_storage;
mod error;
mod genesis_config;
mod global_state;
mod storage;
mod template_storage;

pub use account_storage::AccountStorage;
pub use error::{StorageError, StorageResult};
pub use genesis_config::GenesisConfig;
pub use global_state::GlobalState;
pub use template_storage::TemplateStorage;
