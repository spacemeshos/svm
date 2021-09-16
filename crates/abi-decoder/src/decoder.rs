use svm_abi_layout::layout;
use svm_sdk_std::{safe_try, Result, Vec};
use svm_sdk_types::value::{Primitive, Value};
use svm_sdk_types::{Address, Amount};

use crate::Cursor;

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

pub enum ValueError {
    NotEnoughBytes,
}

/// Denotes a decode error
pub enum DecodeError {
    /// Type decode error
    Type(TypeError),

    /// Value decode error
    Value(ValueError),
}

macro_rules! assert_no_eof {
    ($cursor:expr) => {{
        if $cursor.is_eof() {
            return Result::Err(DecodeError::Type(TypeError::MissingTypeKind));
        }
    }};
}

macro_rules! decode_fixed_primitive {
    ($self:expr, $ty:ident, $n:expr, $iter:expr) => {{
        let ptr = safe_try!($self.read_bytes($iter, $n)).as_ptr();
        let value: $ty = ptr.into();

        let prim = Primitive::$ty(value);
        let value = Value::Primitive(prim);

        Result::Ok(value)
    }};
}

/// Decodes an encoded function buffer back into a `svm_sdk_types::value::Value`
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

        let kind = safe_try!(self.peek_kind(cursor));

        match kind {
            TypeKind::Array => self.decode_composite(cursor),

            _ => self.decode_primitive(cursor),
        }
    }

    fn decode_primitive(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let kind = safe_try!(self.peek_kind(cursor));

        let value = match kind {
            TypeKind::None => safe_try!(self.decode_none(cursor)).into(),
            TypeKind::Unit => safe_try!(self.decode_unit(cursor)).into(),
            TypeKind::Bool => safe_try!(self.decode_bool(cursor)).into(),
            TypeKind::Address => safe_try!(self.decode_addr(cursor)).into(),
            TypeKind::Amount => safe_try!(self.decode_amount(cursor)).into(),
            TypeKind::I8 => safe_try!(self.decode_i8(cursor)).into(),
            TypeKind::U8 => safe_try!(self.decode_u8(cursor)).into(),
            TypeKind::I16 => safe_try!(self.decode_i16(cursor)).into(),
            TypeKind::U16 => safe_try!(self.decode_u16(cursor)).into(),
            TypeKind::I32 => safe_try!(self.decode_i32(cursor)).into(),
            TypeKind::U32 => safe_try!(self.decode_u32(cursor)).into(),
            TypeKind::I64 => safe_try!(self.decode_i64(cursor)).into(),
            TypeKind::U64 => safe_try!(self.decode_u64(cursor)).into(),
            _ => svm_sdk_std::panic(),
        };

        Result::Ok(value)
    }

    fn decode_composite(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let kind = safe_try!(self.peek_kind(cursor));

        let value = match kind {
            TypeKind::Array => safe_try!(self.decode_array(cursor)).into(),

            _ => svm_sdk_std::panic(),
        };

        Result::Ok(value)
    }

    fn decode_none(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        debug_assert_eq!(byte, layout::NONE);

        Result::Ok(Value::none())
    }

    fn decode_unit(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        debug_assert_eq!(byte, layout::UNIT);

        Result::Ok(Value::unit())
    }

    fn decode_bool(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        let v = match byte {
            layout::BOOL_FALSE => false,
            layout::BOOL_TRUE => true,
            _ => svm_sdk_std::panic(),
        };

        Result::Ok(v.into())
    }

    fn decode_addr(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        debug_assert_eq!(byte, layout::ADDRESS);

        decode_fixed_primitive!(self, Address, Address::len(), cursor)
    }

    fn decode_amount(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        let nbytes = match byte {
            layout::AMOUNT_1B => 1,
            layout::AMOUNT_2B => 2,
            layout::AMOUNT_3B => 3,
            layout::AMOUNT_4B => 4,
            layout::AMOUNT_5B => 5,
            layout::AMOUNT_6B => 6,
            layout::AMOUNT_7B => 7,
            layout::AMOUNT_8B => 8,
            _ => svm_sdk_std::panic(),
        };

        let num = safe_try!(self.read_num(cursor, nbytes));
        let amount = Amount(num);

        Result::Ok(amount.into())
    }

    fn decode_i8(&self, cursor: &mut Cursor) -> Result<i8, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        debug_assert!(byte == layout::I8 || byte == layout::U8);

        let num = safe_try!(self.read_num(cursor, 1)) as i8;

        Result::Ok(num)
    }

    fn decode_u8(&self, cursor: &mut Cursor) -> Result<u8, DecodeError> {
        let num = safe_try!(self.decode_i8(cursor)) as u8;

        Result::Ok(num)
    }

    fn decode_i16(&self, cursor: &mut Cursor) -> Result<i16, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        let nbytes = match byte {
            layout::I16_1B | layout::U16_1B => 1,
            layout::I16_2B | layout::U16_2B => 2,
            _ => svm_sdk_std::panic(),
        };

        let num = safe_try!(self.read_num(cursor, nbytes)) as i16;

        Result::Ok(num)
    }

    fn decode_u16(&self, cursor: &mut Cursor) -> Result<u16, DecodeError> {
        let num = safe_try!(self.decode_i16(cursor)) as u16;

        Result::Ok(num)
    }

    fn decode_i32(&self, cursor: &mut Cursor) -> Result<i32, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        let nbytes = match byte {
            layout::I32_1B | layout::U32_1B => 1,
            layout::I32_2B | layout::U32_2B => 2,
            layout::I32_3B | layout::U32_3B => 3,
            layout::I32_4B | layout::U32_4B => 4,
            _ => svm_sdk_std::panic(),
        };

        let num = safe_try!(self.read_num(cursor, nbytes)) as i32;

        Result::Ok(num)
    }

    fn decode_u32(&self, cursor: &mut Cursor) -> Result<u32, DecodeError> {
        let num = safe_try!(self.decode_i32(cursor)) as u32;

        Result::Ok(num)
    }

    fn decode_i64(&self, cursor: &mut Cursor) -> Result<i64, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        let nbytes = match byte {
            layout::I64_1B | layout::U64_1B => 1,
            layout::I64_2B | layout::U64_2B => 2,
            layout::I64_3B | layout::U64_3B => 3,
            layout::I64_4B | layout::U64_4B => 4,
            layout::I64_5B | layout::U64_5B => 5,
            layout::I64_6B | layout::U64_6B => 6,
            layout::I64_7B | layout::U64_7B => 7,
            layout::I64_8B | layout::U64_8B => 8,
            _ => svm_sdk_std::panic(),
        };

        let num = safe_try!(self.read_num(cursor, nbytes)) as i64;

        Result::Ok(num)
    }

    fn decode_u64(&self, cursor: &mut Cursor) -> Result<u64, DecodeError> {
        let num = safe_try!(self.decode_i64(cursor)) as u64;

        Result::Ok(num)
    }

    fn decode_array(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        assert_no_eof!(cursor);

        let len = safe_try!(self.read_byte(cursor));

        let mut values: Vec<Value> = Vec::with_capacity(len as usize);

        let len = match len {
            layout::ARR_0 => 0,
            layout::ARR_1 => 1,
            layout::ARR_2 => 2,
            layout::ARR_3 => 3,
            layout::ARR_4 => 4,
            layout::ARR_5 => 5,
            layout::ARR_6 => 6,
            layout::ARR_7 => 7,
            layout::ARR_8 => 8,
            layout::ARR_9 => 9,
            layout::ARR_10 => 10,
            _ => svm_sdk_std::panic(),
        };
        seq_macro::seq!(n in 0..11 {
            if len > n {
                let value = safe_try!(self.decode_primitive(cursor));
                values.push(value);
            }
        });

        let values: Value = values.into();

        Result::Ok(values)
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

        let slice = safe_try!(self.read_bytes(cursor, nbytes));
        let mut data = [0u8; 8];

        seq_macro::seq!(i in 0..8 {
            if nbytes > i {
                data[7 - i] = slice[nbytes - i - 1];
            }
        });

        Result::Ok(u64::from_be_bytes(data))
    }

    #[inline]
    fn read_bytes<'a>(
        &self,
        cursor: &'a mut Cursor,
        nbytes: usize,
    ) -> Result<&'a [u8], DecodeError> {
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
    fn peek_kind(&self, cursor: &mut Cursor) -> Result<TypeKind, DecodeError> {
        let byte = safe_try!(self.peek(cursor));

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
            | layout::ARR_7
            | layout::ARR_8
            | layout::ARR_9
            | layout::ARR_10 => TypeKind::Array,

            _ => svm_sdk_std::panic(),
        };

        Result::Ok(kind)
    }
}
