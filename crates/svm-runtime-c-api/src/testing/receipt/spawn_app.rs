use svm_codec::api::raw::{decode_func_args, decode_version};
use svm_codec::nibble::NibbleIter;
use svm_common::State;
use svm_types::{AppAddr, WasmValue};

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
        ctor_returns: Vec<WasmValue>,

        /// The gas used during the transaction
        gas_used: u64,
    },

    /// Receipt failed
    Failure {
        /// The reason for failure
        error: String,
    },
}

/// Decodes an encoded receipt into `ClientAppReceipt`. Used for testing
pub fn decode_app_receipt(bytes: &[u8]) -> ClientAppReceipt {
    let mut iter = NibbleIter::new(bytes);

    let version = decode_version(&mut iter).unwrap();
    debug_assert_eq!(0, version);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut iter);

            ClientAppReceipt::Failure { error }
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut iter);
            let init_state = helpers::decode_state(&mut iter);
            let ctor_returns = decode_func_args(&mut iter).unwrap();
            let gas_used = helpers::decode_gas_used(&mut iter);

            ClientAppReceipt::Success {
                addr: addr.into(),
                init_state,
                gas_used,
                ctor_returns,
            }
        }
        _ => unreachable!(),
    }
}
