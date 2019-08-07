#![deny(missing_docs)]
#![allow(unused)]

//! The `svm-contract` crate is responsible on storing and retrieving contracts backed by a database.

mod mem_code_hash_store;
mod parse;
mod traits;
mod types;
mod wire_contract;

/// Parsing a deploy contract transaction
pub use crate::parse::parse_contract;
