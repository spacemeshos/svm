use std::io::{Cursor, Read};

use crate::ParseError;

/// A trait to be implemented by Decoders
pub trait ReadExt: Sized {
    /// Tries to read the next byte, but doesn't move the cursor forward.
    fn seek_byte(&self) -> Option<u8>;

    /// Reads bytes until `buf` is full.
    fn read_fill(&mut self, buf: &mut [u8]) -> Result<(), ParseError> {
        for byte in buf.iter_mut() {
            *byte = self.read_byte()?;
        }

        Ok(())
    }

    /// Reads a single byte
    fn read_byte(&mut self) -> Result<u8, ParseError>;

    /// Reads `length` bytes
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, ParseError> {
        let mut buf = vec![0; length];
        self.read_fill(&mut buf[..])?;
        Ok(buf)
    }
}

/// A trait to be implemented by Encoders
pub trait WriteExt {
    /// Writes a single byte
    fn write_byte(&mut self, byte: u8);

    /// Writes `length` bytes
    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }
}

impl ReadExt for Cursor<&[u8]> {
    fn seek_byte(&self) -> Option<u8> {
        self.get_ref().get(self.position() as usize).copied()
    }

    fn read_byte(&mut self) -> Result<u8, ParseError> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).map_err(|_| ParseError::Eof)?;

        Ok(buf[0])
    }
}

impl WriteExt for Vec<u8> {
    fn write_byte(&mut self, byte: u8) {
        self.push(byte);
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}
