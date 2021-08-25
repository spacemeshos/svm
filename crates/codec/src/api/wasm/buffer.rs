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
pub struct Buffer(Vec<u8>);

impl Buffer {
    const LEN: Range<usize> = 0..4;
    const CAPACITY: Range<usize> = 4..8;
    const DATA: RangeFrom<usize> = 8..;

    const HEADER_SIZE: usize = 8;

    const OK_MARKER: u8 = 1;
    const ERR_MARKER: u8 = 0;

    pub fn alloc(len: u32) -> Self {
        let mut buf = Self(vec![0; len as usize + Self::HEADER_SIZE]);

        buf.set_capacity(len);
        buf.set_len(len);

        buf
    }

    pub fn alloc_ok(data: &[u8]) -> Self {
        let len = u32::try_from(data.len()).unwrap() + 1u32;
        let mut buf = Self::alloc(len);

        buf.as_mut()[0] = Self::OK_MARKER;
        buf.as_mut()[1..].clone_from_slice(data);

        buf
    }

    pub fn alloc_err(string: impl ToString) -> Self {
        let string = string.to_string();
        let len = u32::try_from(string.as_bytes().len()).unwrap() + 1u32;
        let mut buf = Self::alloc(len);

        buf.as_mut()[0] = Self::ERR_MARKER;
        buf.as_mut()[1..].clone_from_slice(string.as_bytes());

        buf
    }

    pub fn free(self) {
        let _ = self.0;
    }

    pub fn offset(&self) -> usize {
        self.0.as_ptr() as usize
    }

    pub unsafe fn from_offset(offset: usize) -> Self {
        let ptr = offset as *mut u8;
        let header = Self(Vec::from_raw_parts(
            ptr,
            Self::HEADER_SIZE,
            Self::HEADER_SIZE,
        ));

        Self(Vec::from_raw_parts(
            ptr,
            header.len() as usize,
            header.capacity() as usize,
        ))
    }

    pub fn len(&self) -> u32 {
        u32::from_be_bytes((&self.0[Self::LEN]).try_into().unwrap())
    }

    pub fn capacity(&self) -> u32 {
        u32::from_be_bytes((&self.0[Self::CAPACITY]).try_into().unwrap())
    }

    pub fn set_len(&mut self, len: u32) {
        assert!(len <= self.capacity());

        self.0[Self::LEN].clone_from_slice(&len.to_be_bytes());
    }

    pub fn set_capacity(&mut self, capacity: u32) {
        assert!(capacity as usize <= self.0.len());

        self.0[Self::CAPACITY].clone_from_slice(&capacity.to_be_bytes());
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.0[Self::DATA]
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[Self::DATA]
    }
}

/// Leaking memory is the default behavior. Freeing is explicit via
/// [`Buffer::free()`].
impl Drop for Buffer {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn len_of_new_buf() {
        assert_eq!(Buffer::alloc(0).len(), 0);
        assert_eq!(Buffer::alloc(1).len(), 1);
        assert_eq!(Buffer::alloc(16).len(), 16);
        assert_eq!(Buffer::alloc(u16::MAX as u32).len(), u16::MAX as u32);
    }

    #[test]
    #[should_panic]
    fn excessive_len_of_new_buf() {
        Buffer::alloc(u32::MAX);
    }

    #[test]
    fn offset_and_from_offset() {
        let buf_1 = Buffer::alloc(100);
        let buf_2 = unsafe { Buffer::from_offset(buf_1.offset()) };
        assert_eq!(buf_1, buf_2);
    }

    #[test]
    fn as_ref_eq_as_mut() {
        let mut buf = Buffer::alloc(100);
        assert_eq!(buf.as_ref().to_vec(), buf.as_mut().to_vec());
    }

    #[test]
    fn set_capacity_ok() {
        let mut buf = Buffer::alloc(140);
        buf.set_capacity(140);
        buf.set_capacity(0);
    }

    #[test]
    #[should_panic]
    fn set_capacity_too_small() {
        let mut buf = Buffer::alloc(100);
        buf.set_capacity(99);
    }
}