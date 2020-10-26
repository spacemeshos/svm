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
mod array;
mod boolean;
mod num_i16;
mod num_i32;
mod num_i64;
mod num_i8;

pub use address::*;
pub use amount::*;
pub use array::*;
pub use boolean::*;
pub use num_i16::*;
pub use num_i32::*;
pub use num_i64::*;

extern crate alloc;
use alloc::vec::Vec;

use crate::traits::Encoder;

use svm_sdk_types::value::{Composite, Primitive, Value};

impl<T> Encoder for &Option<T>
where
    T: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        match self {
            None => encode_none(w),
            Some(v) => v.encode(w),
        }
    }
}

impl<T> Encoder for Option<T>
where
    T: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        (&self).encode(w)
    }
}

impl Encoder for &Value<'_> {
    fn encode(&self, w: &mut Vec<u8>) {
        do_encode(self, w)
    }
}

impl Encoder for Value<'_> {
    #[inline]
    fn encode(&self, w: &mut Vec<u8>) {
        (&self).encode(w)
    }
}

fn do_encode(value: &Value<'_>, w: &mut Vec<u8>) {
    match value {
        Value::Primitive(p) => encode_primitive(p, w),
        Value::Composite(c) => encode_composite(c, w),
    }
}

fn encode_primitive(p: &Primitive, w: &mut Vec<u8>) {
    match p {
        Primitive::None => encode_none(w),
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

fn encode_composite(c: &Composite, w: &mut Vec<u8>) {
    match c {
        Composite::Array(values) => {
            let values: Vec<&dyn Encoder> = values.iter().map(|v| v as &dyn Encoder).collect();
            values.encode(w);
        }
        Composite::ArrayOwned(values) => {
            let values: Vec<&dyn Encoder> = values.iter().map(|v| v as &dyn Encoder).collect();
            values.encode(w);
        }
    }
}

fn encode_none(w: &mut Vec<u8>) {
    use svm_abi_layout::layout;

    w.push(layout::NONE);
}
