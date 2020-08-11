#![no_std]
#![feature(exclusive_range_pattern)]

//! This crate is responsible of encoding SVM types (its actual type and their values to be precise),
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

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;
use alloc::vec::Vec;

use svm_sdk::{
    types::{Primitive, PrimitiveMarker},
    value::{Address, AddressOwned},
};

mod layout;
mod traits;
mod types;

pub use traits::Encoder;

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

// impl_primitive_encoder!(Address<'_>, marker::ADDRESS);
// impl_primitive_encoder!(AddressOwned, marker::ADDRESS);

// impl<'a, T> Encoder for &[T]
// where
//     T: Encoder + PrimitiveMarker,
// {
//     fn encode(&self, buf: &mut Vec<u8>) {
//         // buf.push(marker::ARRAY_START);

//         for elem in self.iter() {
//             elem.encode(buf);
//         }

//         // buf.push(marker::ARRAY_END);
//     }
// }

// impl<'a, T> Encoder for Vec<T>
// where
//     T: Encoder + PrimitiveMarker,
// {
//     #[inline]
//     fn encode(&self, buf: &mut Vec<u8>) {
//         (&self[..]).encode(buf)
//     }
// }
