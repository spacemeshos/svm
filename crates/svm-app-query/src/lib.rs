#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! The purpose of this `svm-app-query` is to represent a schema for an `AppStorage`.
//! Consumers of the crate will be SVM clients that want to give a better user-experience to its end-users.
//!
//! The obvious usage use-case is a UX Wallet that needs to render the App's current state in a friendly way.

/// Rendering related stuff.
pub mod render;

/// Contains Schema related stuff (a.k.a `ABI`).
pub mod schema;

/// Contains Query-related stuff.
pub mod query;
