/// Used for traversal of the encoded function buffer.
/// It will be used by the `Decoder`.
///
/// By having the isolation between the `Decoder` and `Cursor` we are able
/// to execute a `Decoder` method that receives as a parameter `&mut Cursor`  
/// while the `Decoder` is borrowed `&self`.
///
/// This separation was born out of a need to comply to the safe Rust ownership rules
/// (see the look under the `decode_array` under `Decoder` as an example).
pub struct Cursor {
    /// Pointer to the traversed bytes
    pub bytes: *const u8,

    /// The current pointed-by offset
    pub offset: usize,

    /// Number of bytes pointed-by `bytes`
    pub length: usize,
}

impl Cursor {
    /// Creates a new `Cursor` for encoded function buffer `bytes`
    pub fn new(bytes: &[u8]) -> Self {
        let length = bytes.len();
        let bytes = bytes.as_ptr();

        Self {
            bytes,
            length,
            offset: 0,
        }
    }

    /// Returns whether cursor has finished traversal
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.offset >= self.len()
    }

    /// The length of the underlying buffer
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns the next looked-at byte without incrementing `offset`
    /// If already pointing at `EOF` - returns `None`.
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }

        let byte = unsafe { *self.offset_ptr() };
        Some(byte)
    }

    /// Returns the next looked-at byte and increments the `offset`.
    /// If already pointing at `EOF` - returns `None`.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = self.peek();
        self.offset += 1;

        byte
    }

    /// If there are at laest `nbytes` unprocessed-yet bytes,
    /// returns a raw pointer to the current pointed-by address.
    /// And then, it increments the `offset` by `nbytes`.
    ///
    /// In case there are less then `nbytes` left bytes - returns `None`.
    pub fn read_bytes(&mut self, nbytes: usize) -> Option<*const u8> {
        let last_byte_off = self.offset + nbytes - 1;

        if last_byte_off >= self.len() {
            return None;
        }

        let ptr = unsafe { self.offset_ptr() };
        self.offset += nbytes;

        Some(ptr)
    }

    /// Returns a raw pointer to the current pointed-at address.
    #[inline]
    pub unsafe fn offset_ptr(&self) -> *const u8 {
        self.bytes.add(self.offset)
    }
}
