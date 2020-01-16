use crate::buffer::Buffer;

#[derive(Debug, PartialEq, Clone)]
pub struct BufferMut {
    bytes: Vec<u8>,
}

impl BufferMut {
    pub fn new(cap: i32) -> Self {
        Self {
            bytes: Vec::with_capacity(cap as usize),
        }
    }

    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn len(&self) -> i32 {
        self.bytes.len() as i32
    }

    pub fn read(&self, offset: i32, len: i32) -> &[u8] {
        assert!(offset >= 0, "`offset` must be a non-negative number");
        assert!(len > 0, "`len` must be a positive number");

        let start = offset as usize;
        let end = start + (len - 1) as usize;

        assert!(end < self.bytes.len(), "out-of-bounds");

        &self.bytes[start..=end]
    }

    pub fn write(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    pub fn freeze(self) -> Buffer {
        Buffer::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "`offset` must be a non-negative number")]
    fn buffer_start_negative_offset() {
        let cap = 10;
        let mut buf = BufferMut::new(cap);

        buf.read(-1, 5);
    }

    #[test]
    #[should_panic(expected = "`len` must be a positive number")]
    fn buffer_len_negative_offset() {
        let cap = 10;
        let mut buf = BufferMut::new(cap);

        buf.read(0, -1);
    }

    #[test]
    #[should_panic(expected = "out-of-bounds")]
    fn buffer_read_out_of_bounds() {
        let cap = 10;
        let mut buf = BufferMut::new(cap);

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
