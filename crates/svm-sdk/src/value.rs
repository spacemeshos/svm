use core::cmp::PartialEq;
use core::fmt::{self, Debug};

use crate::Amount;

use core::char;

extern crate alloc;

use alloc::vec::Vec;

macro_rules! impl_slice_primitive {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            #[allow(missing_docs)]
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn offset(&self) -> usize {
                self.as_ptr() as _
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            #[allow(missing_docs)]
            #[inline]
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
        }
    };
}

macro_rules! impl_fixed_primitive {
    ($ty:ident, $ty_owned:ident, $nbytes:expr) => {
        #[allow(missing_docs)]
        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $ty<'a>(pub &'a [u8; $nbytes]);

        impl<'a> $crate::types::PrimitiveMarker for $ty<'a> {}

        #[allow(missing_docs)]
        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $ty_owned(pub [u8; $nbytes]);

        impl $crate::types::PrimitiveMarker for $ty_owned {}

        impl_slice_primitive!($ty);

        impl<'a> $ty<'a> {
            /// Size in bytes
            pub const fn size() -> usize {
                $nbytes
            }

            /// Creates a new type with cloned data
            pub fn to_owned(&self) -> $ty_owned {
                let bytes = self.0.clone();

                $ty_owned(bytes)
            }
        }

        impl $ty_owned {
            /// Size in bytes
            pub const fn size() -> usize {
                $nbytes
            }

            /// Returns a type containing borrowed data
            pub fn deref(&self) -> $ty {
                $ty(&self.0)
            }
        }

        impl<'a> From<&'a [u8]> for $ty<'a> {
            fn from(bytes: &'a [u8]) -> Self {
                assert_eq!(bytes.len(), $nbytes);

                let bytes = unsafe { core::mem::transmute::<*const u8, _>(&bytes[0]) };

                $ty(bytes)
            }
        }

        impl From<&[u8]> for $ty_owned {
            fn from(bytes: &[u8]) -> Self {
                let ty: $ty = bytes.into();
                ty.to_owned()
            }
        }

        impl From<Vec<u8>> for $ty_owned {
            fn from(bytes: Vec<u8>) -> Self {
                (&bytes[..]).into()
            }
        }

        impl fmt::Display for $ty<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for byte in self.0.iter() {
                    let (a, b) = byte_as_chars(*byte);
                    write!(f, "{}{}", a, b);
                }

                Ok(())
            }
        }

        impl fmt::Display for $ty_owned {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let ty: $ty = self.deref();

                <$ty<'_> as fmt::Display>::fmt(&ty, f)
            }
        }
    };
}

fn byte_as_chars(byte: u8) -> (char, char) {
    let msb: u8 = (byte & 0xF0) >> 4;
    let lsb: u8 = byte & 0x0F;

    let a = char::from_digit(msb as u32, 16).unwrap();
    let b = char::from_digit(lsb as u32, 16).unwrap();

    (a, b)
}

impl_fixed_primitive!(Address, AddressOwned, 20);

/// Array value
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T>(pub &'a [T]);

/// Primitive value
#[derive(Debug, PartialEq)]
pub enum Primitive<'a> {
    Bool(bool),

    Address(Address<'a>),

    Amount(Amount),

    I8(i8),

    U8(u8),

    I16(i16),

    U16(u16),

    I32(i32),

    U32(u32),

    I64(i64),

    U64(u64),
}

/// Composite value
#[derive(Debug, PartialEq)]
pub enum Composite<'a> {
    /// An `Array`
    Array(&'a [Value<'a>]),
}

/// A value
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    /// A `Primitive` value
    Primitive(Primitive<'a>),

    /// A `Composite` value
    Composite(Composite<'a>),
}

macro_rules! impl_from_num_to_value {
    ($prim_ty:ident, $rust_ty:ident) => {
        impl From<$rust_ty> for Value<'_> {
            fn from(num: $rust_ty) -> Self {
                let prim = Primitive::$prim_ty(num);
                Value::Primitive(prim)
            }
        }
    };
}

impl_from_num_to_value!(I8, i8);
impl_from_num_to_value!(U8, u8);

impl_from_num_to_value!(I16, i16);
impl_from_num_to_value!(U16, u16);

impl_from_num_to_value!(I32, i32);
impl_from_num_to_value!(U32, u32);

impl_from_num_to_value!(I64, i64);
impl_from_num_to_value!(U64, u64);

impl From<bool> for Value<'_> {
    fn from(v: bool) -> Self {
        let prim = Primitive::Bool(v);
        Value::Primitive(prim)
    }
}

impl From<Amount> for Value<'_> {
    fn from(amount: Amount) -> Self {
        let prim = Primitive::Amount(amount);
        Value::Primitive(prim)
    }
}

impl<'a> From<&'a [Value<'_>]> for Value<'a> {
    fn from(slice: &'a [Value]) -> Self {
        let comp = Composite::Array(slice);
        Value::Composite(comp)
    }
}
