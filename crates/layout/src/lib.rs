#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible on representing an Application's storage variables layout.

mod builder;
mod layout;
mod var;

pub use builder::LayoutBuilder;
pub use layout::Layout;
pub use var::{RawVar, Type, SymbolicVar, Id};
