#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible of representing an App's storage variables `Layout`.

mod builder;
mod fixed;
mod var;

pub use builder::LayoutBuilder;
pub use fixed::FixedLayout;
pub use var::{Id, Primitive, RawVar, SymbolicVar, Type};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutKind {
    Fixed,
    //
    // TODO: In the future:
    // Dynamic
    // <https://github.com/spacemeshos/svm/issues/281>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layout {
    Fixed(FixedLayout),
    //
    // TODO: In the future:
    // Dynamic
    // <https://github.com/spacemeshos/svm/issues/281>
}

impl Layout {
    pub fn kind(&self) -> LayoutKind {
        match self {
            Self::Fixed(..) => LayoutKind::Fixed,
        }
    }

    pub fn as_fixed(&self) -> &FixedLayout {
        match self {
            Self::Fixed(layout) => layout,
        }
    }
}
