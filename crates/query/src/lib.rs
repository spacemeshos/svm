//! The purpose of this `svm-app-query` is to represent a schema for an `AppStorage`.
//! Consumers of the crate will be SVM clients that want to give a better user-experience to its end-users.
//!
//! The obvious usage use-case is a UX Wallet that needs to render the App's current state in a friendly way.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

pub mod query;
pub mod render;
pub mod schema;
