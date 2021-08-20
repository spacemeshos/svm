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
pub use ext::{ReadExt, WriteExt};
pub use field::Field;
pub mod api;
pub mod wasm_ffi;

pub use section::{SectionPreview, SectionsDecoder, SectionsEncoder};

/// Encoding of receipts.
pub mod receipt;

mod error;
pub use error::ParseError;

pub trait Codec: Sized {
    type Error;

    fn encode(&self, w: &mut impl WriteExt);

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error>;

    fn fixed_size() -> Option<usize> {
        None
    }

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

    let mut cursor = std::io::Cursor::new(&encoded[..]);
    let decoded = T::decode(&mut cursor).unwrap();

    assert_eq!(item, decoded);
}
