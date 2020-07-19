use svm_sdk::{
    types::marker,
    value::{self, Address, PubKey256, Value},
};

use crate::cursor::Cursor;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum TypeError {
    MissingTypeKind,

    InvalidTypeKind(u8),

    ProhibitedTypeKind(u8),
}

#[derive(Debug)]
pub enum ValueError {
    NotEnoughBytes,
}

/// Denotes a decode error
#[derive(Debug)]
pub enum DecodeError {
    /// Type decode error
    Type(TypeError),

    /// Value decode error
    Value(ValueError),
}

macro_rules! assert_no_eof {
    ($cursor:expr) => {{
        if $cursor.is_eof() {
            return Err(DecodeError::Type(TypeError::MissingTypeKind));
        }
    }};
}

macro_rules! decode_fixed_primitive {
    ($self:expr, $ty:ident, $n:expr, $cursor:expr) => {{
        let ptr = $self.read_bytes($cursor, $n)?;

        let bytes = unsafe { core::mem::transmute::<*const u8, &[u8; $n]>(ptr) };
        let addr = $ty(bytes);

        let primitive = value::Primitive::$ty(addr);
        let value = Value::Primitive(primitive);

        Ok(value)
    }};
}

/// Decodes an encoded function buffer back into a `sdk_values::Value`
pub struct Decoder;

impl Decoder {
    /// New instance
    pub fn new() -> Self {
        Self {}
    }

    /// Decodes the next `sdk_types::Value` (primitive or composite) and returns it.
    /// Returns `DecodeError` when decode fails.
    pub fn decode_value(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        assert_no_eof!(cursor);

        let byte = self.read_byte(cursor)?;

        let value = match byte {
            marker::ARRAY_START => self.decode_array(cursor)?,
            marker::ADDRESS => self.decode_addr(cursor)?,
            marker::PUBKEY_256 => self.decode_pubkey256(cursor)?,
            marker::ARRAY_END => {
                return Err(DecodeError::Type(TypeError::ProhibitedTypeKind(byte)))
            }
            _ => return Err(DecodeError::Type(TypeError::InvalidTypeKind(byte))),
        };

        Ok(value)
    }

    fn decode_array(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        assert_no_eof!(cursor);

        let mut next_byte = self.peek(cursor)?;
        let mut values = Vec::new();

        while next_byte != marker::ARRAY_END {
            let v = self.decode_value(cursor)?;
            values.push(v);

            next_byte = self.peek(cursor)?;
        }

        let _ = self.read_byte(cursor)?;

        let values = Box::leak(Box::new(values));
        let array = Value::Composite(value::Composite::Array(values));

        self.verify_array(&array)?;

        Ok(array)
    }

    fn decode_addr(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        decode_fixed_primitive!(self, Address, 20, cursor)
    }

    fn decode_pubkey256(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        decode_fixed_primitive!(self, PubKey256, 32, cursor)
    }

    fn verify_array(&self, _value: &Value) -> Result<(), DecodeError> {
        // todo!()
        Ok(())
    }

    #[inline]
    fn read_byte(&self, cursor: &mut Cursor) -> Result<u8, DecodeError> {
        cursor
            .read_byte()
            .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }

    #[inline]
    fn read_bytes<'a>(
        &self,
        cursor: &'a mut Cursor,
        nbytes: usize,
    ) -> Result<*const u8, DecodeError> {
        cursor
            .read_bytes(nbytes)
            .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }

    #[inline]
    fn peek(&self, cursor: &mut Cursor) -> Result<u8, DecodeError> {
        cursor
            .peek()
            .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }
}
