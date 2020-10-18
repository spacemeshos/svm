use core::cmp::PartialEq;
use core::fmt::{self, Debug};

use crate::{Address, Amount};

extern crate alloc;

use alloc::vec::Vec;

/// Array value
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T>(pub &'a [T]);

/// Primitive value
#[derive(Debug, PartialEq)]
pub enum Primitive {
    None,

    Bool(bool),

    Address(Address),

    Amount(Amount),

    I8(i8),

    U8(u8),

    I16(i16),

    U16(u16),

    I32(i32),

    U32(u32),

    I64(i64),

    U64(u64),
}

/// Composite value
#[derive(Debug, PartialEq)]
pub enum Composite<'a> {
    /// An `Array`
    Array(&'a [Value<'a>]),

    ArrayOwned(Vec<Value<'a>>),
}

/// A value
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    /// A `Primitive` value
    Primitive(Primitive),

    /// A `Composite` value
    Composite(Composite<'a>),
}

impl<'a> Value<'a> {
    pub const fn none() -> Value<'static> {
        Value::Primitive(Primitive::None)
    }
}

macro_rules! impl_from_rust_to_value {
    ($prim_ident:ident, $T:ident) => {
        impl From<$T> for Value<'_> {
            fn from(num: $T) -> Self {
                let prim = Primitive::$prim_ident(num);
                Value::Primitive(prim)
            }
        }
    };
}

impl<'a, T> From<Option<T>> for Value<'a>
where
    T: Into<Value<'a>>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            None => Value::Primitive(Primitive::None),
            Some(v) => v.into(),
        }
    }
}

impl_from_rust_to_value!(Bool, bool);
impl_from_rust_to_value!(Amount, Amount);

impl_from_rust_to_value!(I8, i8);
impl_from_rust_to_value!(U8, u8);

impl_from_rust_to_value!(I16, i16);
impl_from_rust_to_value!(U16, u16);

impl_from_rust_to_value!(I32, i32);
impl_from_rust_to_value!(U32, u32);

impl_from_rust_to_value!(I64, i64);
impl_from_rust_to_value!(U64, u64);

impl<'a> From<Address> for Value<'a> {
    fn from(addr: Address) -> Self {
        let addr = Primitive::Address(addr);
        Value::Primitive(addr)
    }
}

impl<'a> From<&'a [Value<'_>]> for Value<'a> {
    fn from(slice: &'a [Value]) -> Self {
        let comp = Composite::Array(slice);
        Value::Composite(comp)
    }
}

impl<'a> From<Vec<Value<'a>>> for Value<'a> {
    fn from(array: Vec<Value<'a>>) -> Value<'a> {
        let comp = Composite::ArrayOwned(array);
        Value::Composite(comp)
    }
}

macro_rules! impl_from_value_to_rust {
    ($prim_ident:ident, $T:ty) => {
        impl From<Value<'_>> for $T {
            fn from(value: Value) -> Self {
                match value {
                    Value::Primitive(Primitive::$prim_ident(v)) => v,
                    _ => unreachable!(),
                }
            }
        }

        impl From<Value<'_>> for Option<$T> {
            fn from(value: Value) -> Self {
                match value {
                    Value::Primitive(Primitive::None) => None,
                    Value::Primitive(Primitive::$prim_ident(v)) => Some(v),
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl_from_value_to_rust!(Bool, bool);
impl_from_value_to_rust!(Amount, Amount);

impl_from_value_to_rust!(I8, i8);
impl_from_value_to_rust!(U8, u8);

impl_from_value_to_rust!(I16, i16);
impl_from_value_to_rust!(U16, u16);

impl_from_value_to_rust!(I32, i32);
impl_from_value_to_rust!(U32, u32);

impl_from_value_to_rust!(I64, i64);
impl_from_value_to_rust!(U64, u64);

impl<'a> From<Value<'a>> for Address {
    fn from(value: Value<'a>) -> Self {
        match value {
            Value::Primitive(Primitive::Address(addr)) => addr,
            _ => unreachable!(),
        }
    }
}

impl From<Value<'_>> for Option<Address> {
    fn from(value: Value<'_>) -> Self {
        match value {
            Value::Primitive(Primitive::None) => None,
            Value::Primitive(Primitive::Address(addr)) => Some(addr),
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_to_rust_array {
    ([] => $($tt:tt)*) => {};
    ([$T:tt $($T_tail:tt)*] => $($tt:tt)*) => {
        impl_to_rust_array!($T => $($tt)*);
        impl_to_rust_array!([$($T_tail)*] => $($tt)*);
    };

    ($T:tt => ) => {};
    ($T:tt => $n:tt $($tt:tt)*) => {
        impl_to_rust_array!(@implement $T $n);
        impl_to_rust_array!($T => $($tt)*);
    };
    (@implement $T:tt $n:tt) => {
        impl<'a> From<Value<'a>> for [$T; $n]
        where Value<'a>: Into<$T>
        {
            fn from(value: Value<'a>) -> Self {
                use core::mem::{size_of, MaybeUninit};

                match value {
                    Value::Composite(Composite::ArrayOwned(mut values)) => {
                        assert_eq!(values.len(), $n);

                        let mut array: [MaybeUninit<$T>; $n] = MaybeUninit::uninit_array();

                        for (i, v) in values.drain(..).enumerate() {
                            array[i] = MaybeUninit::new(v.into());
                        }

                        debug_assert_eq!(size_of::<[MaybeUninit<$T>; $n]>(), size_of::<[$T; $n]>());

                        unsafe { core::mem::transmute::<_, Self>(array) }
                    }
                    _ => unreachable!(),
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_to_rust_array!([
    Amount
    Address
    bool
    i8 u8
    i16 u16
    i32 u32
    i64 u64
] => 1 2 3 4 5 6 7 8 9 10);
