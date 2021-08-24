use std::io::{Cursor, Read};
use std::string::FromUtf8Error;

use svm_types::BytesPrimitive;

use crate::{ParseError, Result};

/// A trait to be implemented by Decoders
pub trait ReadExt {
    fn read_fill(&mut self, buf: &mut [u8]) -> Result<()> {
        for byte in buf.iter_mut() {
            *byte = self.read_byte()?;
        }

        Ok(())
    }

    /// Reads a single byte
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_fill(&mut buf[..])?;
        Ok(buf[0])
    }

    /// Reads `length` bytes
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.read_fill(&mut buf[..])?;
        Ok(buf)
    }

    /// Reads a boolean
    fn read_bool(&mut self) -> Result<bool> {
        let byte = self.read_byte()?;

        match byte {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(ParseError::ReachedEOF),
        }
    }

    /// Reads an unsigned 16-bit integer (Big-Endian)
    fn read_u16_be(&mut self) -> Result<u16> {
        let buf = [0u8; 2];
        self.read_fill(&mut buf[..])?;

        Ok(u16::from_be_bytes(buf))
    }

    /// Reads an unsigned 32-bit integer (Big-Endian)
    fn read_u32_be(&mut self) -> Result<u32> {
        let buf = [0u8; 4];
        self.read_fill(&mut buf[..])?;

        Ok(u32::from_be_bytes(buf))
    }

    /// Reads an unsigned 64-bit integer (Big-Endian)
    fn read_u64_be(&mut self) -> Result<u64> {
        let buf = [0u8; 8];
        self.read_fill(&mut buf[..])?;

        Ok(u64::from_be_bytes(buf))
    }

    /// Reads a UTF-8 String
    fn read_string(&mut self) -> Result<std::result::Result<String, FromUtf8Error>> {
        unimplemented!()
    }

    /// Reads a [`BytesPrimitive`] implementor.
    fn read_bytes_prim<V, const N: usize>(&mut self) -> Result<V>
    where
        V: BytesPrimitive<N>,
    {
        let buf = [0u8; N];
        self.read_fill(&mut buf[..])?;

        Ok(V::new(buf))
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

    /// Writes a boolean
    fn write_bool(&mut self, b: bool) {
        self.write_byte(if b { 1 } else { 0 })
    }

    /// Writes an unsigned 16-bit integer (Big-Endian)
    fn write_u16_be(&mut self, n: u16) {
        self.write_bytes(&n.to_be_bytes());
    }

    /// Writes an unsigned 32-bit integer (Big-Endian)
    fn write_u32_be(&mut self, n: u32) {
        self.write_bytes(&n.to_be_bytes());
    }

    /// Writes an unsigned 64-bit integer (Big-Endian)
    fn write_u64_be(&mut self, n: u64) {
        self.write_bytes(&n.to_be_bytes());
    }

    /// Writes a UTF-8 String
    fn write_string(&mut self, s: &str) {
        self.write_bytes(s.as_bytes());
    }

    /// Writes a [`BytesPrimitive`] implementor.
    fn write_bytes_prim<V, const N: usize>(&mut self, prim: &V)
    where
        V: BytesPrimitive<N>,
    {
        self.write_bytes(&prim.as_ref()[..]);
    }
}

impl ReadExt for Cursor<&[u8]> {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0; 1];

        let _ = self.read_exact(&mut buf);

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
