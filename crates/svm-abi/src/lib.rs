#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! The purpose of this `svm-abi` is to represent a schema for an `AppStorage`.
//! Consumers of the crate will be SVM clients that want to give a better user-experience to its end-users.
//!
//! The obvious usage use-case is a UX Wallet that needs to render the App's current state in a friendly way.

mod render;

/// Contains Schema related stuff (a.k.a `ABI`).
pub mod schema;

/// Contains Query-related stuff.
pub mod query;
