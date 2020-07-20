use crate::api::raw::{decode_func_args, decode_version};
use crate::nibble::NibbleIter;

use svm_types::{receipt::Log, State, WasmValue};

use super::helpers;
use crate::receipt::logs;

/// Used for testing the encoding of `ExecReceipt` back to the client.
#[derive(Debug, PartialEq)]
pub enum ClientExecReceipt {
    /// Receipt succeeded
    Success {
        /// The app new state
        new_state: State,

        /// The values returns by the invoked app as a string
        func_returns: Vec<WasmValue>,

        /// The gas used during the transaction
        gas_used: u64,

        /// The logged entries during the transaction
        logs: Vec<Log>,
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
            let func_returns = decode_func_args(&mut iter).unwrap();
            let gas_used = helpers::decode_gas_used(&mut iter);
            let logs = logs::decode_logs(&mut iter);

            ClientExecReceipt::Success {
                new_state,
                gas_used,
                func_returns,
                logs,
            }
        }
        _ => unreachable!(),
    }
}
