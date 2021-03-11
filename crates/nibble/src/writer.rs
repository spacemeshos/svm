extern crate alloc;

use alloc::vec::Vec;

use crate::{concat_nibbles, nib, Nibble};

/// Nibbles Writer.
pub struct NibbleWriter {
    nibbles: Vec<Nibble>,
}

impl NibbleWriter {
    /// Creates a new writer.
    pub fn new() -> Self {
        Self {
            nibbles: Vec::new(),
        }
    }

    /// Pushes a new nibble to the end of stream
    #[inline]
    pub fn push(&mut self, nib: Nibble) {
        self.nibbles.push(nib);
    }

    /// Appends `nibbles` to the underlying stream.
    pub fn write(&mut self, nibbles: &[Nibble]) {
        for nib in nibbles.iter() {
            self.push(*nib);
        }
    }

    /// Appends `bytes` to the underlying stream.
    /// (each byte consists of 2 nibbles).
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.write_byte(*byte);
        }
    }

    /// Appends a `byte` to the underlying stream.
    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        let lnib = nib!((byte & 0xF0) >> 4);
        let rnib = nib!(byte & 0x0F);

        self.write(&[lnib, rnib]);
    }

    /// Closes the `NibbleWriter` and returns the underlying streams as `Vec<u8>`.
    /// In case the number of nibbles is odd, pads a zero-nibble. (see also: `is_byte_aligned`).
    #[must_use]
    #[inline]
    pub fn into_bytes(mut self) -> Vec<u8> {
        // before calling `self.bytes()` we must make sure
        // that its number of nibbles is even. If it's not, we pad it with one extra nibble.

        if self.is_byte_aligned() == false {
            let padding = nib!(0);
            self.write(&[padding]);
        }

        let (bytes, rem) = concat_nibbles(&self.nibbles);
        debug_assert!(rem.is_none());

        bytes
    }

    /// The returns the number of nibbles written so far.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.nibbles.len()
    }

    /// Returns whether the number of written nibbles so far is even.
    #[must_use]
    #[inline]
    pub fn is_byte_aligned(&self) -> bool {
        self.len() % 2 == 0
    }
}
