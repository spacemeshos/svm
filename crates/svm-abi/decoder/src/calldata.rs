use core::iter::Iterator;

use crate::{Cursor, DecodeError, Decoder};

use svm_sdk::value::Value;

pub struct CallData {
    cursor: Cursor<'static>,

    decoder: Decoder,
}

impl CallData {
    pub fn new(bytes: &'static [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            decoder: Decoder::new(),
        }
    }
}

impl Iterator for CallData {
    type Item = Value<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_eof() {
            return None;
        }

        let value = self.decoder.decode_value(&mut self.cursor);

        match value {
            Err(err) => panic!("Invalid CallData"),
            Ok(value) => Some(value),
        }
    }
}

impl CallData {
    pub fn next_2<T1, T2>(&mut self) -> (T1, T2)
    where
        T1: From<Value<'static>>,
        T2: From<Value<'static>>,
    {
        let v1 = self.next().unwrap();
        let v2 = self.next().unwrap();

        (v1.into(), v2.into())
    }

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
