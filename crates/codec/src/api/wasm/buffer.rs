use std::convert::{TryFrom, TryInto};
use std::ops::{Range, RangeFrom};

/// ## WASM Buffer Layout
///
/// Each WASM Buffer contains 2 section: `Header` and `Data`
///
/// +--------------------------------+
/// | Header Section |  Data Section |
/// +--------------------------------+
///
///
/// ### WASM Buffer Header Section
///
/// Each Buffer is prefixed with `Header` consisting of 8 bytes.
///
/// The first 4 bytes are the byte-length of the buffer.
/// The remaining 4 bytes are the capacity byte-length of the buffer.
///
/// The reason we need both `length` and `capacity` and due to the implementation
/// of Rust `Vec`. Even though we use `Vec::with_capacity` we still prefer to store
/// explicitly in the `Header` the `capacity` returned by `Vec#into_raw_parts`.
///
/// See also `Vec#reserve_exact` documentation:
///
/// ```md
/// Note that the allocator may give the collection more space than it
/// requests. Therefore, capacity can not be relied upon to be precisely
/// minimal. Prefer `reserve` if future insertions are expected.
/// ```
///
/// #### WASM Buffer Header Layout
///
/// +------------------+--------------------+
/// | length (4 bytes) | capacity (4 bytes) |
/// +------------------+--------------------+
///
/// Both `length` and `capacity` are laid out in Big-Endian order
///
///
/// ## WASM Buffer Data Section
///
/// Contains the raw data of the buffer.
///
/// Allocates a new WASM buffer having `Data` of `length` bytes.
///
/// The total allocation size of the buffer will always be bigger due to the
/// `Header` section.
/// If for the `capacity` of the `Data` will be bigger - it will also increase
/// the amount of allocated data.
#[derive(Debug, PartialEq)]
pub struct Buffer(Option<Vec<u8>>, bool);

impl Buffer {
    const LEN: Range<usize> = 0..4;
    const CAPACITY: Range<usize> = 4..8;
    const DATA: RangeFrom<usize> = 8..;

    const HEADER_SIZE: usize = 8;

    const OK_MARKER: u8 = 1;
    const ERR_MARKER: u8 = 0;

    pub fn alloc(len: u32) -> Self {
        let mut vec = vec![0; len as usize + Self::HEADER_SIZE];
        vec.shrink_to_fit();

        debug_assert_eq!(vec.capacity(), len as usize + Self::HEADER_SIZE);
        debug_assert_eq!(vec.len(), len as usize + Self::HEADER_SIZE);

        let mut buf = Self(Some(vec), true);

        buf.set_capacity(len);
        buf.set_len(len);

        buf
    }

    #[cfg(test)]
    pub fn then_free(mut self) -> Self {
        self.1 = false;
        self
    }

    pub fn alloc_ok(data: &[u8]) -> Self {
        let len = u32::try_from(data.len()).unwrap() + 1u32;
        let mut buf = Self::alloc(len);

        buf.as_mut()[0] = Self::OK_MARKER;
        buf.as_mut()[1..].clone_from_slice(data);

        buf
    }

    #[allow(unused)]
    pub fn as_result(&self) -> Option<Result<&[u8], &str>> {
        let first_byte = self.as_ref().get(0)?;
        match *first_byte {
            Self::OK_MARKER => Some(Ok(&self.as_ref()[1..])),
            Self::ERR_MARKER => {
                let s = std::str::from_utf8(&self.as_ref()[1..]).ok()?;
                Some(Err(s))
            }
            _ => None,
        }
    }

    pub fn alloc_err(string: impl ToString) -> Self {
        let string = string.to_string();
        let len = u32::try_from(string.as_bytes().len()).unwrap() + 1u32;
        let mut buf = Self::alloc(len);

        buf.as_mut()[0] = Self::ERR_MARKER;
        buf.as_mut()[1..].clone_from_slice(string.as_bytes());

        buf
    }

    pub fn free(mut self) {
        self.0.take();
    }

    pub fn ptr(&mut self) -> *mut u8 {
        self.vec_mut().as_mut_ptr()
    }

    pub unsafe fn from_ptr(ptr: *mut u8) -> Self {
        let header = Self(
            Some(Vec::from_raw_parts(
                ptr,
                Self::HEADER_SIZE,
                Self::HEADER_SIZE,
            )),
            true,
        );

        Self(
            Some(Vec::from_raw_parts(
                ptr,
                header.len() as usize + Self::HEADER_SIZE,
                header.capacity() as usize + Self::HEADER_SIZE,
            )),
            true,
        )
    }

    fn vec(&self) -> &Vec<u8> {
        self.0.as_ref().unwrap()
    }

    fn vec_mut(&mut self) -> &mut Vec<u8> {
        self.0.as_mut().unwrap()
    }

    pub fn len(&self) -> u32 {
        u32::from_be_bytes((&self.vec()[Self::LEN]).try_into().unwrap())
    }

    pub fn capacity(&self) -> u32 {
        u32::from_be_bytes((&self.vec()[Self::CAPACITY]).try_into().unwrap())
    }

    pub fn set_len(&mut self, len: u32) {
        assert!(len <= self.capacity());

        self.vec_mut()[Self::LEN].clone_from_slice(&len.to_be_bytes());
    }

    pub fn set_capacity(&mut self, capacity: u32) {
        assert!(capacity >= self.len());
        assert!(capacity as usize <= self.vec().len() - Self::HEADER_SIZE);

        self.vec_mut()[Self::CAPACITY].clone_from_slice(&capacity.to_be_bytes());
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.vec()[Self::DATA]
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.vec_mut()[Self::DATA]
    }
}

/// Leaking memory is the default behavior. Freeing is explicit via
/// [`Buffer::free()`].
impl Drop for Buffer {
    fn drop(&mut self) {
        if self.0.is_some() && self.1 {
            // https://doc.rust-lang.org/nomicon/destructors.html
            self.0.take().unwrap().leak();
        }
    }
}

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn len_and_capacity_of_new_buf(len: u16) -> bool {
        let len = len as u32;
        let buf = Buffer::alloc(len).then_free();
        buf.len() == len && buf.capacity() == len
    }

    #[test]
    fn u32_max_len() {
        Buffer::alloc(u32::MAX).then_free();
    }

    #[quickcheck]
    fn ptr_and_from_ptr(mut data: Vec<u8>) -> bool {
        data.truncate(u16::MAX as usize);

        let mut buf_1 = Buffer::alloc(data.len() as u32);
        let buf_2 = unsafe { Buffer::from_ptr(buf_1.ptr()) };

        buf_1.as_mut().clone_from_slice(&data);

        buf_1 == buf_2 && buf_1.as_ref() == &data && buf_2.as_ref() == &data
    }

    #[quickcheck]
    fn free(len: u16) -> bool {
        Buffer::alloc(len as u32).free();
        true
    }

    #[quickcheck]
    fn as_ref_eq_as_mut(len: u16) -> bool {
        let len = len as u32;
        let mut buf = Buffer::alloc(len).then_free();
        buf.as_ref().to_vec() == buf.as_mut().to_vec()
    }

    #[test]
    fn set_capacity_ok() {
        let mut buf = Buffer::alloc(140).then_free();
        buf.set_capacity(140);
        buf.set_len(0);
        buf.set_capacity(0);
    }

    #[test]
    #[should_panic]
    fn set_capacity_too_small() {
        let mut buf = Buffer::alloc(100).then_free();
        assert_eq!(buf.len(), 100);
        buf.set_capacity(99);
    }
}
