use std::io::{Cursor, Read, Result};

pub trait ReadExt {
    fn read_byte(&mut self) -> Result<u8>;

    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>>;

    fn read_u16_be(&mut self) -> Result<u16>;

    fn read_u32_be(&mut self) -> Result<u32>;

    fn read_u64_be(&mut self) -> Result<u64>;
}

impl ReadExt for Cursor<&[u8]> {
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0; 1];

        let _ = self.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(length);

        let _ = self.read_exact(&mut buf)?;

        Ok(buf)
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
}
