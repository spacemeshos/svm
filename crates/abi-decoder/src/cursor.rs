use svm_sdk_std::Option;

/// Used for traversal of the encoded function buffer.
/// It will be used by the `Decoder`.
///
/// By having the isolation between the `Decoder` and `Cursor` we are able
/// to execute a `Decoder` method that receives as a parameter `&mut Cursor`  
/// while the `Decoder` is borrowed `&self`.
///
/// This separation was born out of a need to comply to the safe Rust ownership rules
/// (see the look under the `decode_array` under `Decoder` as an example).
pub struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new `Cursor` for encoded function buffer `bytes`
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    /// Returns whether cursor has finished traversal
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.offset >= self.len()
    }

    /// The length of the underlying buffer
    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns the next looked-at byte without incrementing `offset`
    /// If already pointing at `EOF` - returns `None`.
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.is_eof() {
            Option::None
        } else {
            let byte = unsafe { self.bytes.get_unchecked(self.offset) };
            Option::Some(*byte)
        }
    }

    /// Returns the next looked-at byte and increments the `offset`.
    /// If already pointing at `EOF` - returns `None`.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = self.peek();

        self.offset += 1;

        byte
    }

    /// If there are at least `nbytes` unprocessed-yet bytes,
    /// returns a raw pointer to the current pointed-by address.
    /// And then, it increments the `offset` by `nbytes`.
    ///
    /// In case there are less then `nbytes` left bytes - returns `None`.
    pub fn read_bytes(&mut self, nbytes: usize) -> Option<&'a [u8]> {
        match self.bytes.get(self.offset..self.offset + nbytes) {
            core::option::Option::Some(slice) => {
                self.offset += nbytes;
                Option::Some(slice)
            }
            core::option::Option::None => Option::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_whole_contents() {
        let mut cursor = Cursor::new(b"foo");
        assert_eq!(cursor.read_bytes(3), Option::Some(b"foo" as &[u8]));
    }

    #[test]
    fn read_empty() {
        let mut cursor = Cursor::new(b"");
        assert_eq!(cursor.read_bytes(0), Option::Some(b"" as &[u8]));
    }

    #[test]
    fn read_too_many_bytes() {
        let mut cursor = Cursor::new(b"");
        assert_eq!(cursor.read_bytes(1), Option::None);
    }
}
