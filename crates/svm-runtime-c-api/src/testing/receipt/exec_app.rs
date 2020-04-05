use svm_common::State;

use svm_app::raw::{decode_func_args, decode_version, NibbleIter};

use super::helpers;

/// Used for testing the encoding of `ExecReceipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientExecReceipt {
    /// Receipt succeeded
    Success {
        /// The app new state
        new_state: State,

        /// The values returns by the invoked app as a string
        func_returns: String,

        /// The gas used during the transaction
        gas_used: u64,
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
    let mut iter = NibbleIter::new(bytes);

    let version = decode_version(&mut iter).unwrap();
    debug_assert_eq!(0, version);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut iter);
            ClientExecReceipt::Failure { error }
        }
        1 => {
            // success
            let new_state = helpers::decode_state(&mut iter);
            let returns = decode_func_args(&mut iter).unwrap();
            let gas_used = helpers::decode_gas_used(&mut iter);

            ClientExecReceipt::Success {
                new_state,
                gas_used,
                func_returns: helpers::wasm_values_str(&returns[..]),
            }
        }
        _ => unreachable!(),
    }
}
