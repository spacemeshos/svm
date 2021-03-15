use crate::{Cursor, Decoder};

use svm_sdk_std::{Option, Result};
use svm_sdk_types::value::Value;

/// `CallData` exposes an ergonomic API for decoding a binary `calldata`.
///
/// Its main usage is by the `svm-sdk` crate for decoding the binary `calldata` into a Rust native values.
pub struct CallData {
    cursor: Cursor,

    decoder: Decoder,
}

impl CallData {
    /// New instance, input is the binary `calldata` to be decoded.
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),

            decoder: Decoder::new(),
        }
    }
}

/// `CallData` implements the `Iterator` trait.
/// Thus, calling `next` should return the next decoded `Value` (`svm_sdk_types::value::Value`)
///
/// # Safety
/// Since it's assumed that the binary input to `CallData` is valid it aborts when input is in when input is invalid.
///
/// # Example
///
/// ```rust,no_run
/// use svm_abi_decoder::CallData;
///
/// let bytes = vec![];
///
/// let mut calldata = CallData::new(&bytes);
/// let value = calldata.next().unwrap();
/// ```
///
impl CallData {
    pub fn next(&mut self) -> Option<Value> {
        if self.cursor.is_eof() {
            return Option::None;
        }

        let value = self.decoder.decode_value(&mut self.cursor);

        match value {
            Result::Err(..) => core::intrinsics::abort(),
            Result::Ok(value) => Option::Some(value),
        }
    }

    /// Decodes the next `calldata` value, and returns it as type `T1`.
    ///
    /// # Safety
    ///
    /// Panics if there is no next `Value` to decode or if the input is invalid.
    /// Also aborts if the decoded `Value` cannot be converted into a `T1` Rust type.
    ///
    /// ```rust, no_run
    /// use svm_abi_decoder::CallData;
    ///
    /// let bytes = vec![];
    ///
    /// let mut calldata = CallData::new(&bytes);
    /// let num: u8 = calldata.next_1();
    /// ```
    pub fn next_1<T1>(&mut self) -> T1
    where
        T1: From<Value>,
    {
        let v = self.next().unwrap();

        v.into()
    }

    /// Decodes the next `calldata` value, and returns it as tuple of `(T1, T2)`.
    ///
    /// # Safety
    ///
    /// Panics if there are less than two `Value`s to be decoded.
    /// Also aborts if the first decoded `Value` cannot be converted into a `T1` Rust type.
    /// (or if the second decoded `Value` cannot be converted into a `T2` Rust type).
    pub fn next_2<T1, T2>(&mut self) -> (T1, T2)
    where
        T1: From<Value>,
        T2: From<Value>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();

        (v1.into(), v2.into())
    }

    /// Please read the documentation for the above `next_2`
    /// (the `next_3` is extended for an additional `Value`)
    pub fn next_3<T1, T2, T3>(&mut self) -> (T1, T2, T3)
    where
        T1: From<Value>,
        T2: From<Value>,
        T3: From<Value>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();
        let v3 = self.next().unwrap();

        (v1.into(), v2.into(), v3.into())
    }

    /// Please read the documentation for the above `next_2`
    /// (the `next_4` is extended for an additional `Value`s)
    pub fn next_4<T1, T2, T3, T4>(&mut self) -> (T1, T2, T3, T4)
    where
        T1: From<Value>,
        T2: From<Value>,
        T3: From<Value>,
        T4: From<Value>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();
        let v3 = self.next().unwrap();
        let v4 = self.next().unwrap();

        (v1.into(), v2.into(), v3.into(), v4.into())
    }

    /// Please read the documentation for the above `next_2`
    /// (the `next_5` is extended for an additional `Value`s)
    pub fn next_5<T1, T2, T3, T4, T5>(&mut self) -> (T1, T2, T3, T4, T5)
    where
        T1: From<Value>,
        T2: From<Value>,
        T3: From<Value>,
        T4: From<Value>,
        T5: From<Value>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();
        let v3 = self.next().unwrap();
        let v4 = self.next().unwrap();
        let v5 = self.next().unwrap();

        (v1.into(), v2.into(), v3.into(), v4.into(), v5.into())
    }

    /// Please read the documentation for the above `next_2`
    /// (the `next_6` is extended for an additional `Value`s)
    pub fn next_6<T1, T2, T3, T4, T5, T6>(&mut self) -> (T1, T2, T3, T4, T5, T6)
    where
        T1: From<Value>,
        T2: From<Value>,
        T3: From<Value>,
        T4: From<Value>,
        T5: From<Value>,
        T6: From<Value>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();
        let v3 = self.next().unwrap();
        let v4 = self.next().unwrap();
        let v5 = self.next().unwrap();
        let v6 = self.next().unwrap();

        (
            v1.into(),
            v2.into(),
            v3.into(),
            v4.into(),
            v5.into(),
            v6.into(),
        )
    }
}
