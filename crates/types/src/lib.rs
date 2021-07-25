//! This crate contains types that are used throughout the SVM project.
//! Whenever a type has a usage that exceeds a local crate then it should be considered a candidate for this crate.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(vec_into_raw_parts)]

#[macro_use]
mod macros;

mod account;
mod address;
mod address_of;
mod error;
mod spawn_account;
mod state;
mod template;
mod transaction;
mod wasm_type;
mod wasm_value;

/// Type for failed running transactions
pub use error::RuntimeError;

/// Gas-related types
mod gas;
pub use gas::{Gas, GasMode, OOGError};

/// `Receipt`-related types
mod receipt;

pub use receipt::{
    into_spawn_receipt, CallReceipt, DeployReceipt, Receipt, ReceiptLog, ReceiptRef, SpawnReceipt,
};

/// Address-related types
pub use address::{AccountAddr, Address, DeployerAddr, SpawnerAddr, TemplateAddr};
pub use address_of::AddressOf;

pub use account::Account;
pub use spawn_account::SpawnAccount;
pub use state::State;
pub use template::{
    ApiSection, CodeKind, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection,
    SchemaSection, SectionLike, SectionKind, Section, Sections, SectionsIter, Template,
};
pub use transaction::{Context, Envelope, Layer, Transaction, TransactionId};
pub use wasm_type::{WasmType, WasmTypeError};
pub use wasm_value::WasmValue;

/// Represents a type in one of two ways:
/// * `(std::any::TypeId, &'static str str)`
///
/// * `&'static str`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    /// An integer (`std::any::TypeId`) along it a static string.
    /// This string will usually be the value of `std::any::type_name::<T>()`
    TypeId(std::any::TypeId, &'static str),

    /// A static string.
    /// It enables the API user to attach descriptive names as types.
    ///
    /// One can name instances of the same Rust native `struct/enum`
    /// using different strings.  It makes it easier to troubleshoot
    /// leaking resources since we can pinpoint each resource.
    Str(&'static str),
}

use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Str(s) => write!(f, "{}", s),
            Type::TypeId(_ty, s) => write!(f, "{}", s),
        }
    }
}

impl Type {
    /// Creates a `Type` out of generic type (the `T`)
    pub const fn of<T: 'static>() -> Self {
        let ty = std::any::TypeId::of::<T>();
        let name = std::any::type_name::<T>();

        Type::TypeId(ty, name)
    }
}

impl From<&'static str> for Type {
    fn from(s: &'static str) -> Self {
        Type::Str(s)
    }
}
