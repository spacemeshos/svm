use svm_abi_layout::layout;
use svm_sdk::value::{self, Address, Value};

use crate::Cursor;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum TypeError {
    MissingTypeKind,

    InvalidTypeKind(u8),

    ProhibitedTypeKind(u8),
}

enum TypeKind {
    Bool,
    Address,
    Amount,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    Array,
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
    ($self:expr, $ty:ident, $n:expr, $iter:expr) => {{
        let ptr = $self.read_bytes($iter, $n)?;

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

        let kind = self.read_type_kind(cursor)?;

        match kind {
            TypeKind::Address => self.decode_addr(cursor)?,
            _ => todo!(),
        };

        // let value = match byte {
        //     layout::ARRAY_START => self.decode_array(cursor)?,
        //     layout::ADDRESS => self.decode_addr(cursor)?,
        //     layout::ARRAY_END => {
        //         return Err(DecodeError::Type(TypeError::ProhibitedTypeKind(byte)))
        //     }
        //     _ => return Err(DecodeError::Type(TypeError::InvalidTypeKind(byte))),
        // };

        // Ok(value)

        todo!()
    }

    fn decode_array(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        assert_no_eof!(cursor);
        todo!()

        // let mut next_byte = self.peek(cursor)?;
        // let mut values = Vec::new();

        // while next_byte != layout::ARRAY_END {
        //     let v = self.decode_value(cursor)?;
        //     values.push(v);

        //     next_byte = self.peek(cursor)?;
        // }

        // let _ = self.read_byte(cursor)?;

        // let values = Box::leak(Box::new(values));
        // let array = Value::Composite(value::Composite::Array(values));

        // self.verify_array(&array)?;

        // Ok(array)
    }

    fn decode_addr(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        decode_fixed_primitive!(self, Address, 20, cursor)
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

    #[inline]
    fn read_type_kind(&self, cursor: &mut Cursor) -> Result<TypeKind, DecodeError> {
        let byte = self.read_byte(cursor)?;

        let kind = match byte {
            layout::BOOL_FALSE | layout::BOOL_TRUE => TypeKind::Bool,
            layout::ADDRESS => TypeKind::Address,

            layout::AMOUNT_1B
            | layout::AMOUNT_2B
            | layout::AMOUNT_3B
            | layout::AMOUNT_4B
            | layout::AMOUNT_5B
            | layout::AMOUNT_6B
            | layout::AMOUNT_7B
            | layout::AMOUNT_8B => TypeKind::Amount,

            layout::I8 => TypeKind::I8,
            layout::U8 => TypeKind::U8,

            layout::I16_1B | layout::I16_2B => TypeKind::I16,
            layout::U16_1B | layout::U16_2B => TypeKind::U16,

            layout::I32_1B | layout::I32_2B | layout::I32_3B | layout::I32_4B => TypeKind::I32,
            layout::U32_1B | layout::U32_2B | layout::U32_3B | layout::U32_4B => TypeKind::U32,

            layout::I64_1B
            | layout::I64_2B
            | layout::I64_3B
            | layout::I64_4B
            | layout::I64_5B
            | layout::I64_6B
            | layout::I64_7B
            | layout::I64_8B => TypeKind::I64,

            layout::U64_1B
            | layout::U64_2B
            | layout::U64_3B
            | layout::U64_4B
            | layout::U64_5B
            | layout::U64_6B
            | layout::U64_7B
            | layout::U64_8B => TypeKind::U64,

            layout::ARR_0
            | layout::ARR_1
            | layout::ARR_2
            | layout::ARR_3
            | layout::ARR_4
            | layout::ARR_5
            | layout::ARR_6
            | layout::ARR_0_255 => TypeKind::Array,

            _ => unreachable!(),
        };

        Ok(kind)
    }
}
