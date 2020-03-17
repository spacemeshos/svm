use std::{
    convert::TryFrom,
    io::{Cursor, Read},
};

use svm_common::{Address, State};
use svm_runtime::value::Value;

use byteorder::{BigEndian, ReadBytesExt};

use crate::svm_value_type;

pub(crate) fn decode_receipt_error(cursor: &mut Cursor<&[u8]>) -> String {
    let len = cursor.read_u16::<BigEndian>().unwrap() as usize;

    let mut buf = vec![0; len];
    cursor.read_exact(&mut buf[..]).unwrap();

    String::from_utf8(buf).unwrap()
}

pub(crate) fn decode_returns(cursor: &mut Cursor<&[u8]>) -> Vec<Value> {
    let nrets = cursor.read_u8().unwrap() as usize;

    let mut returns = Vec::new();

    for _ in 0..nrets {
        let raw_ty = cursor.read_u8().unwrap();

        let ret = match svm_value_type::try_from(raw_ty) {
            Ok(svm_value_type::SVM_I32) => {
                let value = cursor.read_u32::<BigEndian>().unwrap();
                Value::I32(value)
            }
            Ok(svm_value_type::SVM_I64) => {
                let value = cursor.read_u64::<BigEndian>().unwrap();
                Value::I64(value)
            }
            Err(..) => unreachable!(),
        };

        returns.push(ret);
    }

    returns
}

pub(crate) fn decode_state(cursor: &mut Cursor<&[u8]>) -> State {
    let mut buf = vec![0; State::len()];
    cursor.read(&mut buf);

    State::from(&buf[..])
}

pub(crate) fn decode_address(cursor: &mut Cursor<&[u8]>) -> Address {
    let mut buf = vec![0; Address::len()];
    cursor.read(&mut buf);

    Address::from(&buf[..])
}

pub(crate) fn returns_as_str(returns: &[Value]) -> String {
    let mut buf = String::new();

    for (i, ret) in returns.iter().enumerate() {
        if i != 0 {
            buf.push_str(", ");
        }
        buf.push_str(&format!("{:?}", ret));
    }

    buf
}
