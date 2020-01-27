use std::convert::TryFrom;
use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::svm_value_type;
use svm_common::State;
use svm_runtime::value::Value;

/// Used for testing the encoding of a `Receipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientReceipt {
    /// Receipt succeeded
    Success {
        /// The app new state
        new_state: State,

        /// The values returns by the invoked app as a string
        func_returns: String,
    },

    /// Receipt failed
    Failure {
        /// The reason for failure
        error: String,
    },
}

/// Decodes an encoded receipt into `ClientReceipt`.
/// Used for testing
pub fn decode_receipt(bytes: &[u8]) -> ClientReceipt {
    let mut cursor = Cursor::new(bytes);

    let version = cursor.read_u32::<BigEndian>().unwrap();
    assert_eq!(0, version);

    let is_success = cursor.read_u8().unwrap();

    match is_success {
        0 => {
            // error
            let len = cursor.read_u16::<BigEndian>().unwrap() as usize;

            let mut buf = vec![0; len];
            cursor.read_exact(&mut buf[..]).unwrap();

            let error = String::from_utf8(buf).unwrap();
            ClientReceipt::Failure { error }
        }
        1 => {
            // success
            let mut buf = vec![0; State::len()];
            cursor.read_exact(&mut buf[..]).unwrap();
            let new_state = State::from(&buf[..]);

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

            ClientReceipt::Success {
                new_state,
                func_returns: returns_as_str(&returns[..]),
            }
        }
        _ => unreachable!(),
    }
}

fn returns_as_str(returns: &[Value]) -> String {
    let mut buf = String::new();

    for (i, ret) in returns.iter().enumerate() {
        if i != 0 {
            buf.push_str(", ");
        }
        buf.push_str(&format!("{:?}", ret));
    }

    buf
}
