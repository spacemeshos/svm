use svm_abi_layout::layout;

use svm_sdk_types::value::{Primitive, Value};
use svm_sdk_types::{Address, Amount};

use crate::Cursor;

extern crate alloc;

use alloc::vec::Vec;

#[derive(Debug)]
pub enum TypeError {
    MissingTypeKind,

    InvalidTypeKind(u8),
}

enum TypeKind {
    None,
    Unit,
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
        let value: $ty = ptr.into();

        let prim = Primitive::$ty(value);
        let value = Value::Primitive(prim);

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
    pub fn decode_value<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        assert_no_eof!(cursor);

        let kind = self.read_type_kind(cursor)?;

        let value = match kind {
            TypeKind::None => self.decode_none(cursor)?.into(),
            TypeKind::Unit => self.decode_unit(cursor)?.into(),
            TypeKind::Bool => self.decode_bool(cursor)?.into(),
            TypeKind::Address => self.decode_addr(cursor)?.into(),
            TypeKind::Amount => self.decode_amount(cursor)?.into(),
            TypeKind::I8 => self.decode_i8(cursor)?.into(),
            TypeKind::U8 => self.decode_u8(cursor)?.into(),
            TypeKind::I16 => self.decode_i16(cursor)?.into(),
            TypeKind::U16 => self.decode_u16(cursor)?.into(),
            TypeKind::I32 => self.decode_i32(cursor)?.into(),
            TypeKind::U32 => self.decode_u32(cursor)?.into(),
            TypeKind::I64 => self.decode_i64(cursor)?.into(),
            TypeKind::U64 => self.decode_u64(cursor)?.into(),
            TypeKind::Array => self.decode_array(cursor)?,
        };

        Ok(value)
    }

    fn decode_none<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        let byte = self.read_byte(cursor)?;

        debug_assert_eq!(byte, layout::NONE);

        Ok(Value::none())
    }

    fn decode_unit<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        let byte = self.read_byte(cursor)?;

        debug_assert_eq!(byte, layout::UNIT);

        Ok(Value::unit())
    }

    fn decode_bool<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        let byte = self.read_byte(cursor)?;

        let v = match byte {
            layout::BOOL_FALSE => false,
            layout::BOOL_TRUE => true,
            _ => unreachable!(),
        };

        Ok(v.into())
    }

    fn decode_addr<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        let byte = self.read_byte(cursor)?;

        debug_assert_eq!(byte, layout::ADDRESS);

        decode_fixed_primitive!(self, Address, 20, cursor)
    }

    fn decode_amount<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        let byte = self.read_byte(cursor)?;

        let nbytes = match byte {
            layout::AMOUNT_1B => 1,
            layout::AMOUNT_2B => 2,
            layout::AMOUNT_3B => 3,
            layout::AMOUNT_4B => 4,
            layout::AMOUNT_5B => 5,
            layout::AMOUNT_6B => 6,
            layout::AMOUNT_7B => 7,
            layout::AMOUNT_8B => 8,
            _ => unreachable!(),
        };

        let num = self.read_num(cursor, nbytes)?;
        let amount = Amount(num);

        Ok(amount.into())
    }

    fn decode_i8(&self, cursor: &mut Cursor) -> Result<i8, DecodeError> {
        let byte = self.read_byte(cursor)?;

        debug_assert!(byte == layout::I8 || byte == layout::U8);

        let num = self.read_num(cursor, 1)? as i8;
        Ok(num)
    }

    fn decode_u8(&self, cursor: &mut Cursor) -> Result<u8, DecodeError> {
        let num = self.decode_i8(cursor)? as u8;
        Ok(num)
    }

    fn decode_i16(&self, cursor: &mut Cursor) -> Result<i16, DecodeError> {
        let byte = self.read_byte(cursor)?;

        let nbytes = match byte {
            layout::I16_1B | layout::U16_1B => 1,
            layout::I16_2B | layout::U16_2B => 2,
            _ => unreachable!(),
        };

        let num = self.read_num(cursor, nbytes)? as i16;
        Ok(num)
    }

    fn decode_u16(&self, cursor: &mut Cursor) -> Result<u16, DecodeError> {
        let num = self.decode_i16(cursor)? as u16;
        Ok(num)
    }

    fn decode_i32(&self, cursor: &mut Cursor) -> Result<i32, DecodeError> {
        let byte = self.read_byte(cursor)?;

        let nbytes = match byte {
            layout::I32_1B | layout::U32_1B => 1,
            layout::I32_2B | layout::U32_2B => 2,
            layout::I32_3B | layout::U32_3B => 3,
            layout::I32_4B | layout::U32_4B => 4,
            _ => unreachable!(),
        };

        let num = self.read_num(cursor, nbytes)? as i32;
        Ok(num)
    }

    fn decode_u32(&self, cursor: &mut Cursor) -> Result<u32, DecodeError> {
        let num = self.decode_i32(cursor)? as u32;
        Ok(num)
    }

    fn decode_i64(&self, cursor: &mut Cursor) -> Result<i64, DecodeError> {
        let byte = self.read_byte(cursor)?;

        let nbytes = match byte {
            layout::I64_1B | layout::U64_1B => 1,
            layout::I64_2B | layout::U64_2B => 2,
            layout::I64_3B | layout::U64_3B => 3,
            layout::I64_4B | layout::U64_4B => 4,
            layout::I64_5B | layout::U64_5B => 5,
            layout::I64_6B | layout::U64_6B => 6,
            layout::I64_7B | layout::U64_7B => 7,
            layout::I64_8B | layout::U64_8B => 8,
            _ => unreachable!(),
        };

        let num = self.read_num(cursor, nbytes)? as i64;
        Ok(num)
    }

    fn decode_u64(&self, cursor: &mut Cursor) -> Result<u64, DecodeError> {
        let num = self.decode_i64(cursor)? as u64;
        Ok(num)
    }

    fn decode_array<'a>(&self, cursor: &mut Cursor) -> Result<Value<'a>, DecodeError> {
        assert_no_eof!(cursor);

        let byte = self.read_byte(cursor)?;
        let nitems = match byte {
            layout::ARR_0 => 0,
            layout::ARR_1 => 1,
            layout::ARR_2 => 2,
            layout::ARR_3 => 3,
            layout::ARR_4 => 4,
            layout::ARR_5 => 5,
            layout::ARR_6 => 6,
            layout::ARR_0_255 => self.read_byte(cursor)?,
            _ => unreachable!(),
        };

        let mut values: Vec<Value> = Vec::with_capacity(nitems as usize);

        for _ in 0..nitems {
            let value = self.decode_value(cursor)?;
            values.push(value);
        }

        let values: Value = values.into();
        Ok(values)
    }

    #[inline]
    fn read_byte(&self, cursor: &mut Cursor) -> Result<u8, DecodeError> {
        cursor
            .read_byte()
            .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }

    #[inline]
    fn read_num(&self, cursor: &mut Cursor, nbytes: usize) -> Result<u64, DecodeError> {
        debug_assert!(nbytes > 0 && nbytes <= 8);

        macro_rules! from_be_bytes {
            ($ptr:expr, $nbytes:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                for i in 0..$nbytes {
                    let d = unsafe { *ptr.offset(i as isize) };

                    n = (n << 8) + d as u64;
                }

                n
            }};
        }

        let ptr = self.read_bytes(cursor, nbytes)?;
        let num = from_be_bytes!(ptr, nbytes);

        Ok(num)
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
        let byte = self.peek(cursor)?;

        let kind = match byte {
            layout::NONE => TypeKind::None,
            layout::UNIT => TypeKind::Unit,
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

            _ => {
                extern crate std;

                let err = std::format!("svm-abi-decoder: Unsupported type-kind {}", byte);

                unreachable!(err)
            }
        };

        Ok(kind)
    }
}
