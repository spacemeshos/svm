#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]

//! This crate is responsible on managing an `Account's storage.
//!
//! That includes the specification of an `Account`'s storage-layout.
//! and execution of operations against the an `Account`'s storage (reads and writes).

pub mod account;
pub mod kv;
pub mod testing;
