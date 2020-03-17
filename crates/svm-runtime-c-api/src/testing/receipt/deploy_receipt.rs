use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use svm_app::types::TemplateAddr;

use super::helpers;

/// Used for testing the encoding of `TemplateReceipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientTemplateReceipt {
    /// Receipt succeeded
    Success {
        /// The template address
        addr: TemplateAddr,
    },

    /// Receipt failed
    Failure {
        /// The reason for failure
        error: String,
    },
}

/// Decodes an encoded receipt into `ClientExecReceipt`. Used for testing
pub fn decode_template_receipt(bytes: &[u8]) -> ClientTemplateReceipt {
    let mut cursor = Cursor::new(bytes);

    let version = cursor.read_u32::<BigEndian>().unwrap();
    assert_eq!(0, version);

    let is_success = cursor.read_u8().unwrap();

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut cursor);

            ClientTemplateReceipt::Failure { error }
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut cursor);

            ClientTemplateReceipt::Success { addr: addr.into() }
        }
        _ => unreachable!(),
    }
}
