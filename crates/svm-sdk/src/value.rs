#[repr(transparent)]
pub struct Address<'a>(&'a [u8; 20]);

#[repr(transparent)]
pub struct PubKey256<'a>(&'a [u8; 32]);

#[repr(transparent)]
pub struct Blob<'a>(&'a [u8]);

#[repr(transparent)]
pub struct Array<'a, T>(&'a [T]);

impl<'a, T> Array<'a, T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn as_offset(&self) -> usize {
        self.as_ptr() as _
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.0[..]
    }
}

enum Primitive<'a> {
    // `Blob` of bytes
    Blob(Blob<'a>),

    // An `Address` (20 bytes)
    Address(Address<'a>),

    /// `Public-Key` consisting of 256-bit (32 bytes)
    PubKey256(PubKey256<'a>),
}

enum Composite<'a> {
    Array(&'a [Value<'a>]),
}

enum Value<'a> {
    Primitive(Primitive<'a>),
    Composite(Composite<'a>),
}
