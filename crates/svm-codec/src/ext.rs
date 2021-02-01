use std::io::{Cursor, Read, Result};

use std::string::FromUtf8Error;

use svm_types::{Address, State};

pub trait ReadExt {
    fn read_byte(&mut self) -> Result<u8>;

    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>>;

    fn read_bool(&mut self) -> Result<bool>;

    fn read_u16_be(&mut self) -> Result<u16>;

    fn read_u32_be(&mut self) -> Result<u32>;

    fn read_u64_be(&mut self) -> Result<u64>;

    fn read_string(&mut self) -> Result<std::result::Result<String, FromUtf8Error>>;

    fn read_address(&mut self) -> Result<Address>;

    fn read_state(&mut self) -> Result<State>;
}

pub trait WriteExt {
    fn write_byte(&mut self, byte: u8);

    fn write_bytes(&mut self, bytes: &[u8]);

    fn write_bool(&mut self, b: bool);

    fn write_u16_be(&mut self, n: u16);

    fn write_u32_be(&mut self, n: u32);

    fn write_u64_be(&mut self, n: u64);

    fn write_string(&mut self, s: &str);

    fn write_address(&mut self, addr: &Address);

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
