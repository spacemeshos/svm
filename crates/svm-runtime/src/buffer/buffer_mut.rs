use crate::buffer::Buffer;

/// Read/Write Buffer.
#[derive(Debug, PartialEq, Clone)]
pub struct BufferMut {
    bytes: Vec<u8>,
}

impl BufferMut {
    /// Creates a new buffer with initial capacity set to `cap` parameter.
    pub fn new(cap: u32) -> Self {
        Self {
            bytes: Vec::with_capacity(cap as usize),
        }
    }

    /// Creates a new buffer initialized with `bytes`.
    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Returns the the buffer byte-length.
    pub fn len(&self) -> u32 {
        self.bytes.len() as u32
    }

    /// Returns a slice to buffer underlying bytes `offset, offset + 1, ..., offset + len - 1`
    pub fn read(&self, offset: u32, len: u32) -> &[u8] {
        let start = offset as usize;
        let end = start + (len - 1) as usize;

        assert!(end < self.bytes.len(), "out-of-bounds");

        &self.bytes[start..=end]
    }

    /// Appends `slice` into buffer data.
    pub fn write(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    /// Turns the buffer into a read-only buffer (see: `Buffer`)
    pub fn freeze(self) -> Buffer {
        Buffer::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "out-of-bounds")]
    fn buffer_read_out_of_bounds() {
        let cap = 10;
        let buf = BufferMut::new(cap);

        buf.read(1, 5);
    }

    #[test]
    fn buffer_write_and_read() {
        let cap = 10;
        let mut buf = BufferMut::new(cap);

        buf.write(&[10, 20, 30, 40, 50]);
        assert_eq!(5, buf.len());

        assert_eq!(&[10, 20, 30], buf.read(0, 3));
        assert_eq!(&[20, 30, 40, 50], buf.read(1, 4));
        assert_eq!(&[10, 20, 30, 40, 50], buf.read(0, 5));
    }
}
