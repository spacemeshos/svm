#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! This crate is responsible on representing an Application's storage variables data-layout.

mod builder;
mod layout;

pub use builder::DataLayoutBuilder;
pub use layout::{DataLayout, VarId};
