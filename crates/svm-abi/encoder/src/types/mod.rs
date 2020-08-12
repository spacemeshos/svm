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
//! +------------------------------------+
//! | type (Marker)  | type value (blob) |
//! +------------------------------------+
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

extern crate alloc;
use alloc::vec::Vec;

mod amount;
mod array;
mod boolean;
mod num_i16;
mod num_i32;
mod num_i64;
mod num_i8;

pub use amount::*;
pub use array::*;
pub use boolean::*;
pub use num_i16::*;
pub use num_i32::*;
pub use num_i64::*;

use svm_abi_layout::layout;
use svm_sdk::{
    types::PrimitiveMarker,
    value::{Address, AddressOwned},
};

use crate::Encoder;

macro_rules! impl_primitive_encoder {
    ($ty:ty, $marker:path) => {
        impl Encoder for $ty {
            /// Encodes `self` (of type `$ty`) and outputs the data into `w`
            fn encode(&self, w: &mut Vec<u8>) {
                w.push($marker);

                w.extend_from_slice(&self.0[..]);
            }
        }
    };
}

impl_primitive_encoder!(Address<'_>, layout::ADDRESS);
impl_primitive_encoder!(AddressOwned, layout::ADDRESS);
