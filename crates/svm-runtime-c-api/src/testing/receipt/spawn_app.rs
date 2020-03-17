use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use svm_app::types::AppAddr;
use svm_common::State;

use super::helpers;

/// Used for testing the encoding of `TemplateReceipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientAppReceipt {
    /// Receipt succeeded
    Success {
        /// The app address
        addr: AppAddr,

        /// App's initial state
        init_state: State,

        /// The values returned by the App's ctor, concatenated as a string
        ctor_returns: String,
    },

    /// Receipt failed
    Failure {
        /// The reason for failure
        error: String,
    },
}

/// Decodes an encoded receipt into `ClientAppReceipt`. Used for testing
pub fn decode_app_receipt(bytes: &[u8]) -> ClientAppReceipt {
    let mut cursor = Cursor::new(bytes);

    let version = cursor.read_u32::<BigEndian>().unwrap();
    assert_eq!(0, version);

    let is_success = cursor.read_u8().unwrap();

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut cursor);

            ClientAppReceipt::Failure { error }
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut cursor);
            let init_state = helpers::decode_state(&mut cursor);
            let ctor_returns = helpers::decode_returns(&mut cursor);

            ClientAppReceipt::Success {
                addr: addr.into(),
                init_state,
                ctor_returns: helpers::returns_as_str(&ctor_returns[..]),
            }
        }
        _ => unreachable!(),
    }
}
