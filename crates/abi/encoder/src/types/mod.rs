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

mod address;
mod amount;
mod boolean;
mod num_i16;
mod num_i32;
mod num_i64;
mod num_i8;
mod option;
mod small_array;
mod tuples;
mod unit;

pub use address::*;
pub use amount::*;
pub use boolean::*;
pub use num_i16::*;
pub use num_i32::*;
pub use num_i64::*;
pub use option::*;
pub use small_array::*;
pub use tuples::*;

use crate::traits::Encoder;
use svm_sdk_types::value::{Composite, Primitive, Value};

macro_rules! encode_value {
    ($W:ty) => {
        impl Encoder<$W> for Value {
            #[inline]
            fn encode(&self, w: &mut $W) {
                match self {
                    Value::Primitive(p) => encode_primitive(p, w),
                    Value::Composite(c) => encode_composite(c, w),
                }
            }
        }

        fn encode_primitive(p: &Primitive, w: &mut $W) {
            match p {
                Primitive::None => encode_none(w),
                Primitive::Unit => encode_unit(w),
                Primitive::Address(p) => p.encode(w),
                Primitive::Amount(p) => p.encode(w),
                Primitive::Bool(p) => p.encode(w),
                Primitive::I8(p) => p.encode(w),
                Primitive::U8(p) => p.encode(w),
                Primitive::I16(p) => p.encode(w),
                Primitive::U16(p) => p.encode(w),
                Primitive::I32(p) => p.encode(w),
                Primitive::U32(p) => p.encode(w),
                Primitive::I64(p) => p.encode(w),
                Primitive::U64(p) => p.encode(w),
            }
        }

        fn encode_composite(c: &Composite, w: &mut $W) {
            match c {
                Composite::Vec(values) => {
                    values.as_slice().encode(w);
                }
            }
        }

        #[inline]
        fn encode_none(w: &mut $W) {
            svm_sdk_std::Option::<bool>::None.encode(w)
        }

        #[inline]
        fn encode_unit(w: &mut $W) {
            ().encode(w)
        }
    };
}

encode_value!(svm_sdk_std::Vec<u8>);
