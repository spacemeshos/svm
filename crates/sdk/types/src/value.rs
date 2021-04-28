use crate::{Address, Amount};

use svm_sdk_std::{ensure, panic, Option};

/// Primitive value
#[derive(PartialEq)]
pub enum Primitive {
    None,

    Unit,

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
pub enum Composite {
    Vec(svm_sdk_std::Vec<Value>),
}

pub enum Value {
    /// A `Primitive` value
    Primitive(Primitive),

    /// A `Composite` value
    Composite(Composite),
}

impl Value {
    /// Returns a `Value` representing the ABI `Unit`
    pub const fn unit() -> Value {
        Value::Primitive(Primitive::Unit)
    }

    /// Returns a `Value` representing the ABI `None`
    pub const fn none() -> Value {
        Value::Primitive(Primitive::None)
    }
}

macro_rules! impl_from_rust_to_value {
    ($prim_ident:ident, $T:ident) => {
        impl From<$T> for Value {
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
impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Option::None => Value::Primitive(Primitive::None),
            Option::Some(v) => v.into(),
        }
    }
}

impl From<()> for Value {
    fn from(_val: ()) -> Self {
        Value::unit()
    }
}

impl From<svm_sdk_std::Vec<Value>> for Value {
    fn from(array: svm_sdk_std::Vec<Value>) -> Value {
        let comp = Composite::Vec(array);

        Value::Composite(comp)
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

macro_rules! impl_from_value_to_rust {
    ($prim_ident:ident, $T:ty) => {
        impl From<Value> for $T {
            fn from(value: Value) -> Self {
                match value {
                    Value::Primitive(Primitive::$prim_ident(v)) => v,
                    _ => panic(),
                }
            }
        }

        impl From<Value> for Option<$T> {
            fn from(value: Value) -> Self {
                match value {
                    Value::Primitive(Primitive::None) => Option::None,
                    Value::Primitive(Primitive::$prim_ident(v)) => Option::Some(v),
                    _ => panic(),
                }
            }
        }
    };
}

impl From<Value> for () {
    fn from(value: Value) -> Self {
        match value {
            Value::Primitive(Primitive::Unit) => (),
            _ => panic(),
        }
    }
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
        impl From<Value> for [$T; $n]
        where Value: Into<$T>
        {
            fn from(value: Value) -> Self {
                use core::mem::{size_of, MaybeUninit};

                match value {
                    Value::Composite(Composite::Vec(mut values)) => {
                        ensure!(values.len() == $n);

                        let mut array: [MaybeUninit<$T>; $n] = MaybeUninit::uninit_array();

                        for (i, v) in values.into_iter().enumerate() {
                            array[i] = MaybeUninit::new(v.into());
                        }

                        debug_assert_eq!(size_of::<[MaybeUninit<$T>; $n]>(), size_of::<[$T; $n]>());

                        unsafe { core::mem::transmute::<_, Self>(array) }
                    }
                    _ => panic(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_to_value() {
        let addr1: Address = Address::repeat(0x10);

        let value: Value = addr1.clone().into();
        let addr2: Address = value.into();

        assert_eq!(addr1, addr2);
    }

    #[test]
    fn bool_to_value_and_vice_versa() {
        let value: Value = true.into();
        let truthy: bool = value.into();

        assert_eq!(truthy, true);
    }

    #[test]
    fn i8_to_value_and_vice_versa() {
        let value: Value = 100i8.into();
        let num: i8 = value.into();

        assert_eq!(num, 100i8);
    }

    #[test]
    fn amount_to_value_and_vice_versa() {
        let value: Value = Amount(100).into();
        let amount: Amount = value.into();

        assert_eq!(amount, Amount(100));
    }

    #[test]
    fn option_some_to_value_and_vice_versa() {
        let value: Value = 10i32.into();
        let num: Option<i32> = value.into();

        assert_eq!(num, Option::Some(10i32));
    }

    #[test]
    fn option_none_to_value_and_vice_versa() {
        let value: Value = Value::none();
        let num: Option<i32> = value.into();

        assert_eq!(num, Option::None);
    }

    #[test]
    fn vec_values_to_value_and_vice_versa() {
        use svm_sdk_std::Vec;

        let a: Value = 10u8.into();
        let b: Value = 20u8.into();
        let c: Value = 30u8.into();

        let mut vec = Vec::with_capacity(3);
        vec.push(a);
        vec.push(b);
        vec.push(c);

        let values: Value = vec.into();
    }

    #[test]
    fn unit_to_value_and_vice_versa() {
        let value: Value = ().into();
        let unit: () = value.into();

        assert_eq!(unit, ());
    }
}
