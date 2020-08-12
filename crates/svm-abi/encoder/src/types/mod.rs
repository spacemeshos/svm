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

mod ty_array;
mod ty_bool;
mod ty_i16;
mod ty_i32;
mod ty_i64;
mod ty_i8;

pub use ty_array::*;
pub use ty_bool::*;
pub use ty_i16::*;
pub use ty_i32::*;
pub use ty_i64::*;

use svm_nibble::{nib, NibbleWriter};
use svm_sdk::{
    types::PrimitiveMarker,
    value::{Address, AddressOwned},
};

use crate::{layout, Encoder};

macro_rules! impl_primitive_encoder {
    ($ty:ty, $marker:path) => {
        impl Encoder for $ty {
            /// Encodes `self` (of type `$ty`) and outputs the data into `w`
            fn encode(&self, w: &mut NibbleWriter) {
                w.push(nib!($marker));

                w.write_bytes(&self.0[..]);
            }
        }
    };
}

impl_primitive_encoder!(Address<'_>, layout::ADDRESS);
impl_primitive_encoder!(AddressOwned, layout::ADDRESS);
