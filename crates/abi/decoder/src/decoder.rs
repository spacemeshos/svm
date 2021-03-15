use svm_abi_layout::layout;

#[cfg(feature = "full")]
use std::vec::Vec;

#[cfg(not(feature = "full"))]
use svm_sdk_std::Vec;

use svm_sdk_std::{safe_try, Result};
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
        let ptr = safe_try!($self.read_bytes($iter, $n));
        let value: $ty = ptr.into();

        let prim = Primitive::$ty(value);
        let value = Value::Primitive(prim);

        Result::Ok(value)
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

        let kind = safe_try!(self.read_type_kind(cursor));

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
            TypeKind::Array => safe_try!(self.decode_array(cursor)),
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
            _ => unreachable!(),
        };

        Result::Ok(v.into())
    }

    fn decode_addr(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        let byte = safe_try!(self.read_byte(cursor));

        debug_assert_eq!(byte, layout::ADDRESS);

        decode_fixed_primitive!(self, Address, 20, cursor)
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
            _ => unreachable!(),
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
            _ => unreachable!(),
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
            _ => unreachable!(),
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
            _ => unreachable!(),
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

        macro_rules! impl_decode {
            (0 $cursor:ident $values:ident) => {{
                //
            }};
            (1 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(0 $cursor $values);
            }};
            (2 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(1 $cursor $values);
            }};
            (3 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(2 $cursor $values);
            }};
            (4 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(3 $cursor $values);
            }};
            (5 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(4 $cursor $values);
            }};
            (6 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(5 $cursor $values);
            }};
            (7 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(6 $cursor $values);
            }};
            (8 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(7 $cursor $values);
            }};
            (9 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(8 $cursor $values);
            }};
            (10 $cursor:ident $values:ident) => {{
                impl_decode!(@ $cursor $values);
                impl_decode!(9 $cursor $values);
            }};
            (@ $cursor:ident $values:ident) => {{
                let value = safe_try!(self.decode_value($cursor));

                $values.push(value);
            }}
        }

        let len = safe_try!(self.read_byte(cursor));

        let mut values: Vec<Value> = Vec::with_capacity(len as usize);

        match len {
            layout::ARR_0 => impl_decode!(0 cursor values),
            layout::ARR_1 => impl_decode!(1 cursor values),
            layout::ARR_2 => impl_decode!(2 cursor values),
            layout::ARR_3 => impl_decode!(3 cursor values),
            layout::ARR_4 => impl_decode!(4 cursor values),
            layout::ARR_5 => impl_decode!(5 cursor values),
            layout::ARR_6 => impl_decode!(6 cursor values),
            layout::ARR_7 => impl_decode!(7 cursor values),
            layout::ARR_8 => impl_decode!(8 cursor values),
            layout::ARR_9 => impl_decode!(9 cursor values),
            layout::ARR_10 => impl_decode!(10 cursor values),
            _ => unreachable!(),
        };

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

        macro_rules! from_be_bytes_1 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                n = d0 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_2 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_3 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };
                let d2 = unsafe { *ptr.offset(2) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;
                n = (n << 8) + d2 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_4 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };
                let d2 = unsafe { *ptr.offset(2) };
                let d3 = unsafe { *ptr.offset(3) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;
                n = (n << 8) + d2 as u64;
                n = (n << 8) + d3 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_5 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };
                let d2 = unsafe { *ptr.offset(2) };
                let d3 = unsafe { *ptr.offset(3) };
                let d4 = unsafe { *ptr.offset(4) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;
                n = (n << 8) + d2 as u64;
                n = (n << 8) + d3 as u64;
                n = (n << 8) + d4 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_6 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };
                let d2 = unsafe { *ptr.offset(2) };
                let d3 = unsafe { *ptr.offset(3) };
                let d4 = unsafe { *ptr.offset(4) };
                let d5 = unsafe { *ptr.offset(5) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;
                n = (n << 8) + d2 as u64;
                n = (n << 8) + d3 as u64;
                n = (n << 8) + d4 as u64;
                n = (n << 8) + d5 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_7 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };
                let d2 = unsafe { *ptr.offset(2) };
                let d3 = unsafe { *ptr.offset(3) };
                let d4 = unsafe { *ptr.offset(4) };
                let d5 = unsafe { *ptr.offset(5) };
                let d6 = unsafe { *ptr.offset(6) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;
                n = (n << 8) + d2 as u64;
                n = (n << 8) + d3 as u64;
                n = (n << 8) + d4 as u64;
                n = (n << 8) + d5 as u64;
                n = (n << 8) + d6 as u64;

                n
            }};
        }

        macro_rules! from_be_bytes_8 {
            ($ptr:expr) => {{
                let mut n: u64 = 0;
                let ptr = $ptr as *const u8;

                let d0 = unsafe { *ptr.offset(0) };
                let d1 = unsafe { *ptr.offset(1) };
                let d2 = unsafe { *ptr.offset(2) };
                let d3 = unsafe { *ptr.offset(3) };
                let d4 = unsafe { *ptr.offset(4) };
                let d5 = unsafe { *ptr.offset(5) };
                let d6 = unsafe { *ptr.offset(6) };
                let d7 = unsafe { *ptr.offset(7) };

                n = (n << 8) + d0 as u64;
                n = (n << 8) + d1 as u64;
                n = (n << 8) + d2 as u64;
                n = (n << 8) + d3 as u64;
                n = (n << 8) + d4 as u64;
                n = (n << 8) + d5 as u64;
                n = (n << 8) + d6 as u64;
                n = (n << 8) + d7 as u64;

                n
            }};
        }

        let ptr = safe_try!(self.read_bytes(cursor, nbytes));

        let num = match nbytes {
            1 => from_be_bytes_1!(ptr),
            2 => from_be_bytes_2!(ptr),
            3 => from_be_bytes_3!(ptr),
            4 => from_be_bytes_4!(ptr),
            5 => from_be_bytes_5!(ptr),
            6 => from_be_bytes_6!(ptr),
            7 => from_be_bytes_7!(ptr),
            8 => from_be_bytes_8!(ptr),
            _ => unreachable!(),
        };

        Result::Ok(num)
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

            _ => {
                unreachable!()
            }
        };

        Result::Ok(kind)
    }
}
