#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! This crate is responsible on representing an Application's storage variables layout.

mod builder;
mod layout;

pub use builder::LayoutBuilder;
pub use layout::{Layout, VarId};
