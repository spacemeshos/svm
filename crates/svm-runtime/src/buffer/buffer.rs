use crate::buffer::BufferMut;

/// Readonly `Buffer`.
pub struct Buffer {
    buffer: BufferMut,
}

impl Buffer {
    /// Creates a new `Buffer`, serving as a wrapper for mutable `buffer`.
    pub fn new(buffer: BufferMut) -> Self {
        Self { buffer }
    }

    /// Returns the the buffer byte-length.
    #[inline]
    pub fn len(&self) -> u32 {
        self.buffer.len()
    }

    /// Returns a slice to buffer underlying bytes `offset, offset + 1, ..., offset + len - 1`
    #[inline]
    pub fn read(&self, offset: u32, len: u32) -> &[u8] {
        self.buffer.read(offset, len)
    }
}
