use std::io::{Cursor, Read, Result};
use std::string::FromUtf8Error;

use svm_layout::{Primitive, Type};
use svm_types::{Address, State, TemplateAddr, TransactionId};

/// A trait to be implemented by Decoders
pub trait ReadExt {
    /// Reads a single byte.
    fn read_byte(&mut self) -> Result<u8>;

    /// Reads `length` bytes.
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>>;

    /// Reads a [`bool`] value.
    fn read_bool(&mut self) -> Result<bool>;

    /// Reads an unsigned 16-bit integer (Big-Endian).
    fn read_u16_be(&mut self) -> Result<u16>;

    /// Reads an unsigned 32-bit integer (Big-Endian).
    fn read_u32_be(&mut self) -> Result<u32>;

    /// Reads an unsigned 64-bit integer (Big-Endian).
    fn read_u64_be(&mut self) -> Result<u64>;

    /// Reads an UTF-8 [`String`].
    fn read_string(&mut self) -> Result<std::result::Result<String, FromUtf8Error>>;

    /// Reads an account's [`Address`].
    fn read_address(&mut self) -> Result<Address>;

    /// Reads a [`TemplateAddr`].
    fn read_template_addr(&mut self) -> Result<TemplateAddr>;

    /// Reads a [`State`].
    fn read_state(&mut self) -> Result<State>;

    /// Reads a [`TransactionId`].
    fn read_tx_id(&mut self) -> Result<TransactionId>;

    /// Reads a [`svm_layout::Type`].
    fn read_type_sig(&mut self) -> Result<Option<Type>>;
}

/// A trait to be implemented by Encoders
pub trait WriteExt {
    /// Writes a single byte.
    fn write_byte(&mut self, byte: u8);

    /// Writes `length` bytes.
    fn write_bytes(&mut self, bytes: &[u8]);

    /// Writes a [`bool`] value.
    fn write_bool(&mut self, b: bool);

    /// Writes an unsigned 16-bit integer (Big-Endian).
    fn write_u16_be(&mut self, n: u16);

    /// Writes an unsigned 32-bit integer (Big-Endian).
    fn write_u32_be(&mut self, n: u32);

    /// Writes an unsigned 64-bit integer (Big-Endian).
    fn write_u64_be(&mut self, n: u64);

    /// Writes a UTF-8 [`String`].
    fn write_string(&mut self, s: &str);

    /// Writes an `Account Address`
    fn write_address(&mut self, addr: &Address);

    /// Writes a `Template Address`
    fn write_template_addr(&mut self, addr: &TemplateAddr);

    /// Writes a [`State`].
    fn write_state(&mut self, state: &State);

    /// Writes a [`TransactionId`].
    fn write_tx_id(&mut self, tx: &TransactionId);

    /// Writes a [`svm_layout::Type`].
    fn write_type_sig(&mut self, type_sig: Type);
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

    fn read_template_addr(&mut self) -> Result<TemplateAddr> {
        let bytes = self.read_bytes(TemplateAddr::len())?;
        let addr = bytes.as_slice().into();

        Ok(addr)
    }

    fn read_state(&mut self) -> Result<State> {
        let bytes = self.read_bytes(State::len())?;
        let state = bytes.as_slice().into();

        Ok(state)
    }

    fn read_tx_id(&mut self) -> Result<TransactionId> {
        let bytes = self.read_bytes(TransactionId::len())?;
        let tx_id = bytes.as_slice().into();

        Ok(tx_id)
    }

    fn read_type_sig(&mut self) -> Result<Option<Type>> {
        let byte = self.read_byte()?;
        Ok(u8_to_type_sig(byte))
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
        let byte = if b == false { 0 } else { 1 };

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

    fn write_template_addr(&mut self, addr: &TemplateAddr) {
        let bytes = addr.as_slice();

        self.write_bytes(bytes);
    }

    fn write_state(&mut self, state: &State) {
        let bytes = state.as_slice();

        self.write_bytes(bytes);
    }

    fn write_tx_id(&mut self, tx: &TransactionId) {
        let bytes = tx.as_slice();

        self.write_bytes(bytes);
    }

    fn write_type_sig(&mut self, type_sig: Type) {
        let byte = type_sig_to_u8(type_sig);
        self.write_byte(byte);
    }
}

fn nibble_to_primitive(nibble: u8) -> Option<Primitive> {
    Some(match nibble {
        0x0 => Primitive::Bool,
        0x1 => Primitive::I8,
        0x2 => Primitive::U8,
        0x3 => Primitive::I16,
        0x4 => Primitive::U16,
        0x5 => Primitive::I32,
        0x6 => Primitive::U32,
        0x7 => Primitive::I64,
        0x8 => Primitive::U64,
        0x9 => Primitive::Amount,
        0xa => Primitive::Address,
        _ => return None,
    })
}

fn primitive_to_nibble(prim: Primitive) -> u8 {
    match prim {
        Primitive::Bool => 0x0,
        Primitive::I8 => 0x1,
        Primitive::U8 => 0x2,
        Primitive::I16 => 0x3,
        Primitive::U16 => 0x4,
        Primitive::I32 => 0x5,
        Primitive::U32 => 0x6,
        Primitive::I64 => 0x7,
        Primitive::U64 => 0x8,
        Primitive::Amount => 0x9,
        Primitive::Address => 0xa,
    }
}

fn u8_to_type_sig(byte: u8) -> Option<Type> {
    let ls_nibble = byte & 0xf;
    let ms_nibble = byte >> 4;
    let primitive = nibble_to_primitive(ls_nibble)?;

    Some(match ms_nibble {
        0xf => Type::Primitive(primitive),
        0x0..=0xa => Type::Array {
            primitive,
            length: ms_nibble as usize,
        },
        _ => return None,
    })
}

fn type_sig_to_u8(ty: Type) -> u8 {
    match ty {
        Type::Primitive(prim) => 0xf0 | primitive_to_nibble(prim),
        Type::Array {
            primitive: prim,
            length,
        } => {
            assert!(length <= 10);
            (length << 4) as u8 | primitive_to_nibble(prim)
        }
    }
}
