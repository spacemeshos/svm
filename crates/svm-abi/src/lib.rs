#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! The purpose of this `svm-abi` is to represent a schema for an `AppStorage`.
//! Consumers of the crate will be SVM clients that want to give a better user-experience to its end-users.
//!
//! The obvious usage use-case is a UX Wallet that needs to render the App's current state in a friendly way.

pub mod query;
pub mod render;
pub mod schema;
