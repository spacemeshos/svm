#![no_std]

//! This crate is responsible of encoding SVM types (its actuall type and their values to be precise),
//! according to a simple ABI format.
//! The ABI consists of encoding:
//!
//! * Primitive - Currently only `Address` (20 bytes) and `PublicKey256` (256-bit <=> 32 bytes) are supported.
//!
//! * Composite - Currently only an `Array` of the `Primitive`(s) above is supported.
//!
//! ## Primitive Encoding:
//!
//! #### Fixed-Size
//!
//! +------------------------------------------+
//! | type marker (1 byte) | type value (blob) |
//! +------------------------------------------+
//!
//!
//! ## Composite Encoding:
//!
//! ### Array of primitives
//!
//! +-----------------------------------------------------+-----------------------------------------------------------+
//! | Array Start Marker (1 byte) | Primitive #1 Encoding | . . . | Primitive #N Encoding | Array End Marker (1 byte) |
//! +-----------------------------------------------------------------------------------------------------------------+
//!
//!
//! ### Note:
//!
//! Actually the current `Encoder` code supports encoding also `Array` of `Array`'s but it'll error when decoded
//! (see the `svm-abi-decoder` crate).
//!

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

extern crate alloc;
use crate::alloc::vec::Vec;

use svm_sdk::{
    types::marker,
    value::{Address, AddressOwned, PubKey256, PubKey256Owned},
};

/// A traits to be implemented by of types.
pub trait Encoder {
    /// Encodes `self` and outputs the data into `buf`
    fn encode(&self, buf: &mut Vec<u8>);
}

macro_rules! impl_primitive_encoder {
    ($ty:ty, $marker:path) => {
        impl Encoder for $ty {
            /// Encodes `self` (of type `$ty`) and outputs the data into `buf`
            fn encode(&self, buf: &mut Vec<u8>) {
                buf.push($marker);

                buf.extend_from_slice(&self.0[..])
            }
        }
    };
}

impl_primitive_encoder!(Address<'_>, marker::ADDRESS);
impl_primitive_encoder!(AddressOwned, marker::ADDRESS);

impl_primitive_encoder!(PubKey256<'_>, marker::PUBKEY_256);
impl_primitive_encoder!(PubKey256Owned, marker::PUBKEY_256);

impl<'a, T> Encoder for &[T]
where
    T: Encoder,
{
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::ARRAY_START);

        for elem in self.iter() {
            elem.encode(buf);
        }

        buf.push(marker::ARRAY_END);
    }
}

impl<'a, T> Encoder for Vec<T>
where
    T: Encoder,
{
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        (&self[..]).encode(buf)
    }
}
