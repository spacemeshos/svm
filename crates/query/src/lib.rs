//! The purpose of this `svm-query` crate is to represent a Schema for an `AccountStorage`.
//! Consumers of the crate will be SVM clients that want to give a better user-experience to its end-users.
//!
//! The obvious usage use-case is a Client that needs to render an `Account`'s current storage variables in a friendly way.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

pub mod query;
pub mod render;
pub mod schema;
