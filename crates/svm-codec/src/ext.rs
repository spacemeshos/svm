use std::io::{Cursor, Read, Result};

use std::string::FromUtf8Error;

use svm_types::{Address, State};

/// A trait to be implemented by Decoders
pub trait ReadExt {
    /// Reads a single byte
    fn read_byte(&mut self) -> Result<u8>;

    /// Reads `length` bytes
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>>;

    /// Reads a boolean
    fn read_bool(&mut self) -> Result<bool>;

    /// Reads an unsigned 16-bit integer (Big-Endian)
    fn read_u16_be(&mut self) -> Result<u16>;

    /// Reads an unsigned 32-bit integer (Big-Endian)
    fn read_u32_be(&mut self) -> Result<u32>;

    /// Reads an unsigned 64-bit integer (Big-Endian)
    fn read_u64_be(&mut self) -> Result<u64>;

    /// Reads a UTF-8 String
    fn read_string(&mut self) -> Result<std::result::Result<String, FromUtf8Error>>;

    /// Reads an `Address`
    fn read_address(&mut self) -> Result<Address>;

    /// Reads a `State`
    fn read_state(&mut self) -> Result<State>;
}

/// A trait to be implemented by Encoders
pub trait WriteExt {
    /// Writes a single byte
    fn write_byte(&mut self, byte: u8);

    /// Writes `length` bytes
    fn write_bytes(&mut self, bytes: &[u8]);

    /// Writes a boolean
    fn write_bool(&mut self, b: bool);

    /// Writes an unsigned 16-bit integer (Big-Endian)
    fn write_u16_be(&mut self, n: u16);

    /// Writes an unsigned 32-bit integer (Big-Endian)
    fn write_u32_be(&mut self, n: u32);

    /// Writes an unsigned 64-bit integer (Big-Endian)
    fn write_u64_be(&mut self, n: u64);

    /// Writes a UTF-8 String
    fn write_string(&mut self, s: &str);

    /// Writes an `Address`
    fn write_address(&mut self, addr: &Address);

    /// Writes a `State`
    fn write_state(&mut self, state: &State);
}

impl ReadExt for Cursor<&[u8]> {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0; 1];

        let _ = self.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; length];

        let _ = self.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn read_bool(&mut self) -> Result<bool> {
        let byte = self.read_byte()?;

        let b = byte != 0;

        Ok(b)
    }

    fn read_u16_be(&mut self) -> Result<u16> {
        let mut buf = [0; 2];

        let _ = self.read_exact(&mut buf)?;
        let num = u16::from_be_bytes(buf);

        Ok(num)
    }

    fn read_u32_be(&mut self) -> Result<u32> {
        let mut buf = [0; 4];

        let _ = self.read_exact(&mut buf)?;
        let num = u32::from_be_bytes(buf);

        Ok(num)
    }

    fn read_u64_be(&mut self) -> Result<u64> {
        let mut buf = [0; 8];

        let _ = self.read_exact(&mut buf)?;
        let num = u64::from_be_bytes(buf);

        Ok(num)
    }

    fn read_string(&mut self) -> Result<std::result::Result<String, FromUtf8Error>> {
        let length = self.read_byte()?;
        let bytes = self.read_bytes(length as usize)?;

        let string = String::from_utf8(bytes);

        Ok(string)
    }

    fn read_address(&mut self) -> Result<Address> {
        let bytes = self.read_bytes(Address::len())?;
        let addr = bytes.as_slice().into();

        Ok(addr)
    }

    fn read_state(&mut self) -> Result<State> {
        let bytes = self.read_bytes(State::len())?;
        let state = bytes.as_slice().into();

        Ok(state)
    }
}

impl WriteExt for Vec<u8> {
    fn write_byte(&mut self, byte: u8) {
        self.push(byte);
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn write_bool(&mut self, b: bool) {
        let byte = if (b == false) { 0 } else { 1 };

        self.write_byte(byte);
    }

    fn write_u16_be(&mut self, n: u16) {
        let bytes = n.to_be_bytes();

        self.write_bytes(&bytes[..]);
    }

    fn write_u32_be(&mut self, n: u32) {
        let bytes = n.to_be_bytes();

        self.write_bytes(&bytes[..]);
    }

    fn write_u64_be(&mut self, n: u64) {
        let bytes = n.to_be_bytes();

        self.write_bytes(&bytes[..]);
    }

    fn write_string(&mut self, s: &str) {
        let length = s.len();

        assert!(length <= std::u8::MAX as usize);

        self.write_byte(length as u8);

        let bytes = s.as_bytes();
        self.write_bytes(bytes);
    }

    fn write_address(&mut self, addr: &Address) {
        let bytes = addr.as_slice();

        self.write_bytes(bytes);
    }

    fn write_state(&mut self, state: &State) {
        let bytes = state.as_slice();

        self.write_bytes(bytes);
    }
}
