use core::cmp::PartialEq;
use core::fmt::{self, Debug};

use crate::{Address, Amount};

extern crate alloc;

use alloc::vec::Vec;

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
    /// A borrowed `Array`
    Array(&'a [Value<'a>]),

    /// An owned `Array`
    ArrayOwned(Vec<Value<'a>>),
}

/// An ABI Value
///
/// # Example
///
/// ```rust
/// use svm_sdk_types::Address;
/// use svm_sdk_types::value::Value;
///
/// let addr1: Address = [0x10; Address::len()].into();
///
/// let value: Value = addr1.clone().into();
/// let addr2: Address = value.into();
///
/// assert_eq!(addr1, addr2);
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    /// A `Primitive` value
    Primitive(Primitive),

    /// A `Composite` value
    Composite(Composite<'a>),
}

impl<'a> Value<'a> {
    /// Returns a `Value` representing the ABI `None`
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

/// Since the ABI contains a representation for a missing value (`None`)
/// We can decode ABI data into Rust `Option<T>` when `T : Into<Value<'_>`.
///
/// see later in this file the following macros:
///
/// * `impl_from_rust_to_value`
/// * `impl_from_value_to_rust`
/// * `impl_value_to_rust_array`
///
/// These macros facilitate the `ABI Value <=> Rust Type` conversions.  
/// Here are a few examples (there're more examples in other parts of this file).
///
///
/// # Example (boolean)
///
/// ```rust
/// use svm_sdk_types::value::Value;
///
/// let value: Value = true.into();
/// let truthy: bool = value.into();
///
/// assert_eq!(truthy, true);
/// ```
///
///
/// # Example (`i8` - same for other integers)
///
/// ```rust
/// use svm_sdk_types::value::Value;
///
/// let value: Value = 100i8.into();
/// let num: i8 = value.into();
///
/// assert_eq!(num, 100i8);
/// ```
///
///
/// # Example (`Amount`)
///
/// ```rust
/// use svm_sdk_types::Amount;
/// use svm_sdk_types::value::Value;
///
/// let value: Value = Amount(100).into();
/// let amount: Amount = value.into();
///
/// assert_eq!(amount, Amount(100));
/// ```
///
///
/// # Example (Address)
///
/// ```rust
/// use svm_sdk_types::value::Value;
///
/// let value: Value = 10i32.into();
/// let num: Option<i32> = value.into();
///
/// assert_eq!(num, Some(10i32));
/// ```
///
///
/// # Example
///
/// ```rust
/// use svm_sdk_types::value::Value;
///
/// let value: Value = Value::none();
/// let num: Option<i32> = value.into();
///
/// assert_eq!(num, None);
/// ```
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

impl_from_rust_to_value!(Address, Address);

/// Array value
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Array<'a, T>(pub &'a [T]);

/// Takes `&[Value]` and turns it into
/// a single (`Composite`) Value (of kind `Array`)
///
/// # Example
///
/// ```rust
/// use svm_sdk_types::value::Value;
///
/// let value1: Value = 10u8.into();
/// let value2: Value = 20u8.into();
/// let value3: Value = 30u8.into();
///
/// let vec = vec![value1, value2, value3];
/// let values: Value = vec.as_slice().into();
/// ```
impl<'a> From<&'a [Value<'_>]> for Value<'a> {
    fn from(slice: &'a [Value]) -> Self {
        let comp = Composite::Array(slice);
        Value::Composite(comp)
    }
}

/// Takes a `Vec<Value>` and turns it into
/// a single (`Composite`) Value (of kind `ArrayOwned`)
///
/// # Example
///
/// ```rust
/// use svm_sdk_types::value::Value;
///
/// let value1: Value = 10u8.into();
/// let value2: Value = 20u8.into();
/// let value3: Value = 30u8.into();
///
/// let values: Value = vec![value1, value2, value3].into();
/// ```
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

impl_from_value_to_rust!(Address, Address);

macro_rules! impl_value_to_rust_array {
    ([] => $($tt:tt)*) => {};
    ([$T:tt $($T_tail:tt)*] => $($tt:tt)*) => {
        impl_value_to_rust_array!($T => $($tt)*);
        impl_value_to_rust_array!([$($T_tail)*] => $($tt)*);
    };

    ($T:tt => ) => {};
    ($T:tt => $n:tt $($tt:tt)*) => {
        impl_value_to_rust_array!(@implement $T $n);
        impl_value_to_rust_array!($T => $($tt)*);
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
impl_value_to_rust_array!([
    Amount
    Address
    bool
    i8 u8
    i16 u16
    i32 u32
    i64 u64
] => 1 2 3 4 5 6 7 8 9 10);
