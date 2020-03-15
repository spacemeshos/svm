//!          `Receipt` Raw Format Version 0.0.0.0
//!
//!  On success (`is_success = 1`)
//!  ----------------------------------------------------
//!  |   format   |              |                       |
//!  |  version   |  is_success  |     app new state     |
//!  |  (4 bytes) |   (1 byte)   |      (32 bytes)       |
//!  |____________|______________|_______________________|
//!  |          |              |         |               |
//!  | #returns | ret #1 type  | ret #1  |    . . . .    |
//!  | (1 byte) |  (1 byte)    |  value  |               |
//!  |__________|______________|_________|_______________|
//!
//!
//!  On failure (`is_success = 0`)
//!  -----------------------------------------------------
//!  |   format   |                |                     |
//!  |  version   |  is_success    |     error size      |
//!  |  (4 bytes) |   (1 byte)     |      (2 bytes)      |
//!  |____________|________________|_____________________|
//!  |                                                   |
//!  |            error data (UTF-8 string)              |
//!  |___________________________________________________|
//!

use byteorder::{BigEndian, WriteBytesExt};

use svm_common::State;
use svm_runtime::{error::ExecAppError, value::Value, Receipt};

use crate::svm_value_type;

const PROTO_VER: usize = 4;
const ERROR_LENGTH: usize = 2;
const IS_SUCCESS: usize = 1;
const HEADER: usize = PROTO_VER + IS_SUCCESS;

pub(crate) fn encode_receipt(receipt: &Receipt) -> Vec<u8> {
    let size_hint = receipt_size_hint(receipt);
    let mut buf: Vec<u8> = Vec::with_capacity(size_hint);

    write_header(&mut buf, receipt);

    if receipt.success {
        write_new_state(&mut buf, receipt);
        write_returns(&mut buf, receipt);
    } else {
        write_error(&mut buf, receipt);
    };

    buf
}

fn receipt_size_hint(receipt: &Receipt) -> usize {
    if receipt.success {
        HEADER + State::len() + returns_size_hint(receipt)
    } else {
        HEADER + ERROR_LENGTH + error_size_hint(receipt)
    }
}

fn error_size_hint(_receipt: &Receipt) -> usize {
    // we have no quick way to give a good estimation for the error blob size
    // without actually rendering it first.
    // so we arbitrarily return `1024` as the estimated size required.
    1024
}

fn returns_size_hint(receipt: &Receipt) -> usize {
    let returns_count = receipt_returns_count(receipt);

    // * field `#returns` takes 2 bytes
    // * each return vale is preceded by one byte indicating the value type
    // * worst case every return value occupies 8 bytes (64-bit integer).
    //   so each return value + its type can take at most 9 bytes

    2 + returns_count * 9
}

fn write_header(buf: &mut Vec<u8>, receipt: &Receipt) {
    // TODO: handle each `unwrap()`
    // `version` field. we only have `verson=0` for now.
    buf.write_u32::<BigEndian>(0).unwrap();

    // `is_success` field
    if receipt.success {
        buf.write_u8(1).unwrap();
    } else {
        buf.write_u8(0).unwrap();
    }
}

fn write_new_state(buf: &mut Vec<u8>, receipt: &Receipt) {
    assert!(receipt.success);

    let new_state = receipt.new_state.as_ref().unwrap();

    buf.extend_from_slice(new_state.as_slice());
}

fn write_returns(buf: &mut Vec<u8>, receipt: &Receipt) {
    assert!(receipt.success);

    let returns_count = receipt_returns_count(receipt);

    // asserting that `returns_count` fits into a single byte
    assert!(returns_count <= 0xFF);
    buf.write_u8(returns_count as u8).unwrap();

    let returns = receipt.returns.as_ref().unwrap();

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

fn write_error(buf: &mut Vec<u8>, receipt: &Receipt) {
    let error: &ExecAppError = receipt.error.as_ref().unwrap();

    let error_data = format!("{:?}", error);
    let error_size = error_data.as_bytes().len();

    buf.write_u16::<BigEndian>(error_size as u16).unwrap();
    buf.extend_from_slice(error_data.as_bytes());
}

#[inline]
fn receipt_returns_count(receipt: &Receipt) -> usize {
    receipt.returns.as_ref().unwrap().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{testing, testing::ClientReceipt};

    use svm_common::{Address, State};
    use svm_runtime::{error::ExecAppError, value::Value};

    #[test]
    fn encode_receipt_error() {
        let error = ExecAppError::AppNotFound {
            app_addr: Address::of("my-app").into(),
        };

        let expected = ClientReceipt::Failure {
            error: error.to_string(),
        };

        let receipt = Receipt {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
        };

        let bytes = encode_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_receipt_success_without_returns() {
        let new_state = State::from(0x10_20_30_40);

        let expected = ClientReceipt::Success {
            new_state: new_state.clone(),
            func_returns: "".to_string(),
        };

        let receipt = Receipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(Vec::new()),
        };

        let bytes = encode_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_receipt_success_with_returns() {
        let new_state = State::from(0x10_20_30_40);
        let returns = vec![Value::I32(10), Value::I64(20), Value::I32(30)];

        let expected = ClientReceipt::Success {
            new_state: new_state.clone(),
            func_returns: "I32(10), I64(20), I32(30)".to_string(),
        };

        let receipt = Receipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(returns),
        };

        let bytes = encode_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
