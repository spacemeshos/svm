use core::fmt::Debug;

pub trait Slice {
    fn len(&self) -> usize;

    fn offset(&self) -> usize;

    fn as_slice(&self) -> &[u8];

    #[inline]
    fn as_ptr(&self) -> *const u8;
}

macro_rules! impl_slice_primitive {
    ($ty:ident) => {
        impl<'a> Slice for $ty<'a> {
            #[inline]
            fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            fn offset(&self) -> usize {
                self.as_ptr() as _
            }

            #[inline]
            fn as_ptr(&self) -> *const u8 {
                self.0.as_ptr()
            }

            #[inline]
            fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
        }
    };
}

macro_rules! impl_fixed_primitive {
    ($ty:ident, $size:expr) => {
        #[derive(Debug, PartialEq)]
        #[repr(transparent)]
        pub struct $ty<'a>(pub &'a [u8; $size]);

        impl_slice_primitive!($ty);
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

impl_fixed_primitive!(Address, 20);
impl_fixed_primitive!(PubKey256, 32);

impl_blob_primitive!(Blob1);
impl_blob_primitive!(Blob2);
impl_blob_primitive!(Blob3);

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T>(pub &'a [T]);

impl<'a, T> Array<'a, T> {}

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
