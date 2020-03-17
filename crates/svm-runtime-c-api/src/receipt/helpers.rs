use byteorder::{BigEndian, WriteBytesExt};
use std::io::Read;

use svm_runtime::{receipt::Receipt, value::Value};

use crate::svm_value_type;

pub(crate) fn encode_is_success(buf: &mut Vec<u8>, receipt: &Receipt) {
    // For now, we only have `version=0`
    let version = 0;

    buf.write_u32::<BigEndian>(version).unwrap();

    if receipt.is_success() {
        buf.write_u8(1).unwrap();
    } else {
        buf.write_u8(0).unwrap();
    }
}

pub(crate) fn encode_returns(buf: &mut Vec<u8>, receipt: &Receipt) {
    debug_assert!(receipt.is_success());

    let returns = receipt.get_returns();
    let returns_count = returns.len();

    // asserting that `returns_count` fits into a single byte
    assert!(returns_count <= 0xFF);
    buf.write_u8(returns_count as u8).unwrap();

    for value in returns.iter() {
        match value {
            Value::I32(v) => {
                buf.write_u8(svm_value_type::SVM_I32 as u8).unwrap();
                buf.write_u32::<BigEndian>(*v).unwrap();
            }
            Value::I64(v) => {
                buf.write_u8(svm_value_type::SVM_I64 as u8).unwrap();
                buf.write_u64::<BigEndian>(*v).unwrap();
            }
        }
    }
}
