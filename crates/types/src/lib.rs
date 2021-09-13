//! This crate contains types that are used throughout the SVM project.
//! Whenever a type has a usage that exceeds a local crate then it should be considered a candidate for this crate.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(const_type_id)]
#![feature(const_type_name)]
#![feature(vec_into_raw_parts)]

mod account;
mod address;
mod error;
mod gas;
mod receipt;
mod spawn_account;
mod state;
mod template;
mod transaction;
mod wasm_type;
mod wasm_value;

use std::{convert::TryInto, fmt};

pub use account::Account;
pub use address::{Address, TemplateAddr};
pub use error::RuntimeError;
pub use gas::{Gas, GasMode, OOGError};
pub use receipt::{
    into_spawn_receipt, CallReceipt, DeployReceipt, Receipt, ReceiptLog, ReceiptRef, SpawnReceipt,
};
pub use spawn_account::SpawnAccount;
pub use state::State;
pub use template::{
    ApiSection, CodeKind, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection,
    SchemaSection, Section, SectionKind, SectionLike, Sections, SectionsIter, Template,
};
pub use transaction::{Context, Envelope, Layer, Transaction, TransactionId};
pub use wasm_type::{WasmType, WasmTypeError};
pub use wasm_value::WasmValue;

/// Operations on fixed-size byte array entities.
pub trait BytesPrimitive<const N: usize>:
    From<[u8; N]> + AsRef<[u8; N]> + std::hash::Hash + PartialEq + Eq + Clone
{
    /// The constant number of bytes that make up instances of `Self`.
    const N: usize = N;

    /// Copies the contents of `val` in a new instance of `Self`.
    ///
    /// # Panics
    ///
    /// Panics if `val.as_ref()` is not of the appropriate length.
    fn new<V>(val: V) -> Self
    where
        V: AsRef<[u8]>,
    {
        Self::new_opt(val).expect("Invalid bytes length")
    }

    /// Copies the contents of `val` in a new instance of `Self`; returns None
    /// in case `val.as_ref()` is not of the appropriate length.
    fn new_opt<V>(val: V) -> Option<Self>
    where
        V: AsRef<[u8]>,
    {
        let bytes = val.as_ref().try_into().ok()?;
        Some(Self::from(bytes))
    }

    /// Creates a new `Self` that starts with the UTF-8 byte representation of
    /// `s`; all remaining bytes are set to zero.
    ///
    /// # Panics
    ///
    /// Panics if `s` doesn't fit into `Self`.
    fn of(s: &str) -> Self {
        let bytes = s.as_bytes();
        assert!(bytes.len() <= N);

        let mut buf = [0u8; N];
        (&mut buf[..bytes.len()]).clone_from_slice(bytes);

        Self::from(buf)
    }

    /// Returns a reference to the raw contents of `self`.
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Creates a new `Self` with all bytes set to zero.
    fn zeros() -> Self {
        [0; N].into()
    }

    /// Checks whether or not `self` is equal to [`BytesPrimitive::zeros()`].
    fn is_zeros(&self) -> bool {
        Self::zeros().as_ref() == self.as_ref()
    }

    /// Fills a new instance of [`Self`] with `byte`.
    fn repeat(byte: u8) -> Self {
        [byte; N].into()
    }

    /// Returns an [`Iterator`] over the contents of `Self`.
    fn iter(&self) -> std::slice::Iter<u8> {
        self.as_ref().iter()
    }

    /// Returns an immutable reference to the first `n` bytes.
    fn first_n(&self, n: usize) -> &[u8] {
        assert!(n <= N);

        &self.as_slice()[0..n]
    }

    /// Returns an immutable reference to the last `n` bytes.
    fn last_n(&self, n: usize) -> &[u8] {
        assert!(n <= N);

        &self.as_slice()[N - n..]
    }

    /// formats the primitive as a concatenation of:
    /// * first `first` bytes in hex
    /// * ...
    /// * last `last` bytes in hex
    fn fmt(&self, first: usize, last: usize) -> String {
        let first = self.first_n(first);
        let last = self.last_n(last);

        format!("{}...{}", hex::encode_upper(first), hex::encode_upper(last),)
    }

    /// Returns a [`String`] that contains a human-readable representation of
    /// `self`, using ASCII uppercase hexadecimals.
    fn to_string(&self) -> String {
        hex::encode_upper(self.as_ref())
    }
}

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
