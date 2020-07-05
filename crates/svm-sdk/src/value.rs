pub trait Slice {
    fn len(&self) -> usize;

    fn offset(&self) -> usize;

    fn as_slice(&self) -> &[u8];

    #[inline]
    fn as_ptr(&self) -> *const u8;
}

#[repr(transparent)]
pub struct Address<'a>(pub &'a [u8; 20]);

#[repr(transparent)]
pub struct PubKey256<'a>(pub &'a [u8; 32]);

#[repr(transparent)]
pub struct Blob1<'a>(pub &'a [u8]);

#[repr(transparent)]
pub struct Blob2<'a>(pub &'a [u8]);

#[repr(transparent)]
pub struct Blob3<'a>(pub &'a [u8]);

#[repr(transparent)]
pub struct Array<'a, T>(pub &'a [T]);

impl<'a> Slice for Address<'a> {
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

impl<'a, T> Array<'a, T> {}

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

pub enum Composite<'a> {
    Array(&'a [Value<'a>]),
}

pub enum Value<'a> {
    Primitive(Primitive<'a>),
    Composite(Composite<'a>),
}
