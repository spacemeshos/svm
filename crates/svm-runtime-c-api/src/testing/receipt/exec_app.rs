use std::{
    convert::TryFrom,
    io::{Cursor, Read},
};

use byteorder::{BigEndian, ReadBytesExt};

use svm_common::State;

use super::helpers;
use crate::svm_value_type;

/// Used for testing the encoding of `ExecReceipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientExecReceipt {
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

/// Decodes an encoded receipt into `ClientExecReceipt`.
/// Used for testing
pub fn decode_exec_receipt(bytes: &[u8]) -> ClientExecReceipt {
    let mut cursor = Cursor::new(bytes);

    let version = cursor.read_u32::<BigEndian>().unwrap();
    assert_eq!(0, version);

    let is_success = cursor.read_u8().unwrap();

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut cursor);
            ClientExecReceipt::Failure { error }
        }
        1 => {
            // success
            let new_state = helpers::decode_state(&mut cursor);
            let returns = helpers::decode_returns(&mut cursor);

            ClientExecReceipt::Success {
                new_state,
                func_returns: helpers::returns_as_str(&returns[..]),
            }
        }
        _ => unreachable!(),
    }
}
