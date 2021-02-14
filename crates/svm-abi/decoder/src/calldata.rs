use core::iter::Iterator;

use crate::{Cursor, Decoder};

use svm_sdk_types::value::Value;

/// `CallData` exposes an ergonomic API for decoding a binary `calldata`.
///
/// Its main usage is by the `svm-sdk` crate for decoding the binary `calldata`
/// into an Rust native values.
///
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
/// Since it's assumed that the binary input to `CallData` is valid it panics when input is in when input is invalid.
///
/// # Example
///
/// ```rust,no_run
/// use svm_sdk_decoder::CallData;
///
/// let bytes = vec![];
///
/// let mut calldata = CallData::new(&bytes);
/// let value = CallData::next().unwrap();
/// ```
///
impl Iterator for CallData {
    type Item = Value<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_eof() {
            return None;
        }

        let value = self.decoder.decode_value(&mut self.cursor);

        match value {
            Err(..) => panic!("Invalid `CallData`"),
            Ok(value) => Some(value),
        }
    }
}

impl CallData {
    /// Decodes the next `calldata` value, and returns it as type `T1`.
    ///
    /// # Safety
    ///
    /// Panics if there is no next `Value` to decode or if the input is invalid.
    /// Also panics if the decoded `Value` cannot be converted into a `T1` Rust type.
    pub fn next_1<T1>(&mut self) -> T1
    where
        T1: From<Value<'static>>,
    {
        let v1 = self.next().unwrap();

        v1.into()
    }

    /// Decodes the next `calldata` value, and returns it as tuple of `(T1, T2)`.
    ///
    /// # Safety
    ///
    /// Panics if there are less than two `Value`s to be decoded.
    /// Also panics if the first decoded `Value` cannot be converted into a `T1` Rust type.
    /// (or if the second decoded `Value` cannot be converted into a `T2` Rust type).
    pub fn next_2<T1, T2>(&mut self) -> (T1, T2)
    where
        T1: From<Value<'static>>,
        T2: From<Value<'static>>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();

        (v1.into(), v2.into())
    }

    /// Please read the documentation for the above `next_2`
    /// (the `next_3` is extended for an additional `Value`)
    pub fn next_3<T1, T2, T3>(&mut self) -> (T1, T2, T3)
    where
        T1: From<Value<'static>>,
        T2: From<Value<'static>>,
        T3: From<Value<'static>>,
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
        T1: From<Value<'static>>,
        T2: From<Value<'static>>,
        T3: From<Value<'static>>,
        T4: From<Value<'static>>,
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
        T1: From<Value<'static>>,
        T2: From<Value<'static>>,
        T3: From<Value<'static>>,
        T4: From<Value<'static>>,
        T5: From<Value<'static>>,
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
        T1: From<Value<'static>>,
        T2: From<Value<'static>>,
        T3: From<Value<'static>>,
        T4: From<Value<'static>>,
        T5: From<Value<'static>>,
        T6: From<Value<'static>>,
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
