#![feature(generic_associated_types)]

use core::cmp::PartialEq;
use core::fmt::Debug;
use core::ops::Deref;

extern crate alloc;
use alloc::vec::Vec;

macro_rules! impl_slice_primitive {
    ($ty:ident) => {
        impl<'a> $ty<'a> {
            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            pub fn offset(&self) -> usize {
                self.as_ptr() as _
            }

            #[inline]
            pub fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            #[inline]
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
        }
    };
}

macro_rules! impl_fixed_primitive {
    ($ty:ident, $ty_owned:ident, $nbytes:expr) => {
        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $ty<'a>(pub &'a [u8; $nbytes]);

        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $ty_owned(pub [u8; $nbytes]);

        impl_slice_primitive!($ty);

        impl<'a> $ty<'a> {
            pub const fn size() -> usize {
                $nbytes
            }

            pub fn to_owned(&self) -> $ty_owned {
                let bytes = self.0.clone();

                $ty_owned(bytes)
            }
        }

        impl $ty_owned {
            pub const fn size() -> usize {
                $nbytes
            }

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
    };
}

macro_rules! impl_blob_primitive {
    ($ty:ident) => {
        #[derive(core::fmt::Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $ty<'a>(pub &'a [u8]);

        impl_slice_primitive!($ty);
    };
}

impl_fixed_primitive!(Address, AddressOwned, 20);
impl_fixed_primitive!(PubKey256, PubKey256Owned, 32);

impl_blob_primitive!(Blob1);
impl_blob_primitive!(Blob2);
impl_blob_primitive!(Blob3);

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T>(pub &'a [T]);

#[derive(Debug, PartialEq)]
pub enum Primitive<'a> {
    // `Blob` with `length < 256` bytes
    Blob1(Blob1<'a>),

    // `Blob` with `length < 65,536` bytes
    Blob2(Blob2<'a>),

    // `Blob` with `length < 16,777,216` bytes
    Blob3(Blob3<'a>),

    // An `Address` (20 bytes)
    Address(Address<'a>),

    /// `Public-Key` consisting of 256-bit (32 bytes)
    PubKey256(PubKey256<'a>),
}

#[derive(Debug, PartialEq)]
pub enum Composite<'a> {
    Array(&'a [Value<'a>]),
}

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Primitive(Primitive<'a>),
    Composite(Composite<'a>),
}

impl<'a> Value<'a> {
    pub fn as_addr(self) -> Option<Address<'a>> {
        match self {
            Value::Primitive(Primitive::Address(addr)) => Some(addr),
            _ => None,
        }
    }

    pub fn as_pubkey256(self) -> Option<PubKey256<'a>> {
        match self {
            Value::Primitive(Primitive::PubKey256(pubkey)) => Some(pubkey),
            _ => None,
        }
    }
}
