//! This crate is responsible for doing the binary encoding for SVM transactions.
//! It code is compiled as a single WASM file and it should be integrated by clients (e.g `smapp / CLI Wallet`).
//!
//! By doing that, a client can locally encode a binary transaction without having to re-implement all the logic
//! of the `svm-codec`.
//!
//! SVM's CI emits the WASM package of `svm-codec` as one of its artifacts (`svm_codec.wasm`)

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod codec_impls;
mod ext;
mod field;
mod section;
mod version;

pub mod template;
use std::io::Cursor;

pub use ext::{ReadExt, WriteExt};
pub use field::Field;
pub mod api;
pub mod wasm_ffi;

pub use section::{SectionPreview, SectionsDecoder, SectionsEncoder};

/// Encoding of receipts.
pub mod receipt;

mod error;
pub use error::{ParseError, Result};

/// Ability to encode and decode items of a certain type.
pub trait Codec: Sized {
    /// The type of errors that can arise during decoding operations.
    ///
    /// This should be [`std::convert::Infallible`] if nonexistant.
    type Error;

    /// Writes a binary representation of `self` to `w`.
    fn encode(&self, w: &mut impl WriteExt);

    /// Attempts to parse a binary representation of `Self` pointed at by
    /// `cursor`. Returns a [`Codec::Error`] on failure.
    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> std::result::Result<Self, Self::Error>;

    /// Like [`Codec::decode`], but can be used with anything resembling bytes.
    fn decode_bytes<B>(bytes: B) -> std::result::Result<Self, Self::Error>
    where
        B: AsRef<[u8]>,
    {
        Self::decode(&mut Cursor::new(bytes.as_ref()))
    }

    /// In case `Self` has a binary representation with a fixed size, this
    /// should return [`Some`] with the appropriate size; [`None`] otherwise. It
    /// can be used in pre-allocation optimizations.
    fn fixed_size() -> Option<usize> {
        None
    }

    /// Calls [`Codec::encode`] with an empty [`Vec<u8>`] and immediately
    /// returns it.
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut w = Vec::with_capacity(Self::fixed_size().unwrap_or_default());
        self.encode(&mut w);
        w
    }

    #[cfg(test)]
    fn test_encode_then_decode(&self) {}
}

#[cfg(test)]
fn test_codec<T, E>(item: T)
where
    T: Codec<Error = E> + std::fmt::Debug + PartialEq,
    E: std::fmt::Debug,
{
    let encoded = item.encode_to_vec();

    let decoded = T::decode_bytes(encoded).unwrap();

    assert_eq!(item, decoded);
}
