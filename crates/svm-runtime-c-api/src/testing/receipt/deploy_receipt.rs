use svm_codec::api::raw::decode_version;
use svm_codec::nibble::NibbleIter;
use svm_types::TemplateAddr;

use super::helpers;

/// Used for testing the encoding of `TemplateReceipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientTemplateReceipt {
    /// Receipt succeeded
    Success {
        /// The template address
        addr: TemplateAddr,

        /// gas used
        gas_used: u64,
    },

    /// Receipt failed
    Failure {
        /// The reason for failure
        error: String,
    },
}

/// Decodes an encoded receipt into `ClientExecReceipt`. Used for testing
pub fn decode_template_receipt(bytes: &[u8]) -> ClientTemplateReceipt {
    let mut iter = NibbleIter::new(bytes);

    let version = decode_version(&mut iter).unwrap();
    debug_assert_eq!(0, version);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut iter);

            ClientTemplateReceipt::Failure { error }
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut iter);
            let gas_used = helpers::decode_gas_used(&mut iter);

            ClientTemplateReceipt::Success {
                addr: addr.into(),
                gas_used,
            }
        }
        _ => unreachable!(),
    }
}
