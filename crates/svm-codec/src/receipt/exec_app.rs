//!         `Exec App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +---------------------------------------------------+
//!  |  tx type  |  version   | is_success |  New state  |
//!  | (1 byte)  | (1 nibble) | (1 nibble) | (32 bytes)  |
//!  +___________|____________|__________________________+
//!  |          |              |         |               |
//!  | #returns | ret #1 type  | ret #1  |  ret #2  type |
//!  +__________|______________|_________|_______________+
//!  |          |            |                           |
//!  |  ret #2  |   .  .  .  |         gas_used          |
//!  +__________|____________|___________________________+
//!  |          |            |         |                 |
//!  |  #logs   | log 1 blob |  . . .  |     log #N      |
//!  +__________|____________|_________|_________________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use crate::api::raw;
use crate::nibble::{NibbleIter, NibbleWriter};

use svm_types::gas::MaybeGas;
use svm_types::receipt::{ExecReceipt, Log, Receipt};

use super::{decode_error, encode_error, helpers, logs};

pub fn encode_exec_receipt(receipt: &ExecReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::ExecApp(receipt);

    helpers::encode_type(super::types::EXEC_APP, &mut w);
    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_new_state(receipt, &mut w);
        encode_returns(receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = receipt.get_logs();

        encode_error(receipt.get_error(), logs, &mut w);
    };

    w.into_bytes()
}

pub fn decode_exec_receipt(bytes: &[u8]) -> ExecReceipt {
    let mut iter = NibbleIter::new(bytes);

    let ty = helpers::decode_type(&mut iter);
    debug_assert_eq!(ty, crate::receipt::types::EXEC_APP);

    let version = helpers::decode_version(&mut iter).unwrap();
    debug_assert_eq!(0, version);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            let (err, logs) = decode_error(&mut iter);
            ExecReceipt::from_err(err, logs)
        }
        1 => {
            // success
            let new_state = helpers::decode_state(&mut iter);
            let returns = raw::decode_func_args(&mut iter).unwrap();
            let gas_used = helpers::decode_gas_used(&mut iter);
            let logs = logs::decode_logs(&mut iter);

            ExecReceipt {
                success: true,
                error: None,
                new_state: Some(new_state),
                returns: Some(returns),
                gas_used,
                logs,
            }
        }
        _ => unreachable!(),
    }
}

fn encode_new_state(receipt: &ExecReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let new_state = receipt.get_new_state();

    helpers::encode_state(&new_state, w);
}

fn encode_returns(receipt: &ExecReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let returns = receipt.get_returns();
    helpers::encode_returns(&returns, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::{gas::MaybeGas, receipt::ReceiptError, Address, State, WasmValue};

    #[test]
    fn encode_decode_exec_receipt_error() {
        let app = Address::of("my-app");
        let error = ReceiptError::AppNotFound(app.into());

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = ExecReceipt {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
            logs,
        };

        let bytes = encode_exec_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_exec_app(), receipt);
    }

    #[test]
    fn encode_decode_exec_receipt_success_without_returns() {
        let new_state = State::of("some-state");

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = ExecReceipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(Vec::new()),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_exec_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_exec_app(), receipt);
    }

    #[test]
    fn encode_decode_exec_receipt_success_with_returns() {
        let new_state = State::of("some-state");
        let returns = vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)];

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = ExecReceipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(returns),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_exec_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_exec_app(), receipt);
    }
}
