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

use crate::nibble::NibbleWriter;
use svm_types::receipt::{ExecReceipt, Log, Receipt};

use super::{encode_error, helpers, logs::encode_logs};

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
        encode_logs(&receipt.logs, &mut w);
    } else {
        encode_error(&wrapped_receipt, &mut w);
    };

    w.into_bytes()
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

    use crate::receipt::testing::{self, ClientExecReceipt};

    use svm_types::receipt::error::ExecAppError;
    use svm_types::{gas::MaybeGas, Address, State, WasmValue};

    #[test]
    fn encode_decode_exec_receipt_error() {
        let error = ExecAppError::AppNotFound {
            app_addr: Address::of("my-app").into(),
        };

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let expected = ClientExecReceipt::Failure {
            error: error.to_string(),
        };

        let receipt = ExecReceipt {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
            logs,
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_exec_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_exec_receipt_success_without_returns() {
        let new_state = State::of("some-state");

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let expected = ClientExecReceipt::Success {
            new_state: new_state.clone(),
            func_returns: Vec::new(),
            gas_used: 100,
            logs: logs.clone(),
        };

        let receipt = ExecReceipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(Vec::new()),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_exec_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_exec_receipt_success_with_returns() {
        let new_state = State::of("some-state");
        let returns = vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)];

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let expected = ClientExecReceipt::Success {
            new_state: new_state.clone(),
            func_returns: vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)],
            gas_used: 100,
            logs: logs.clone(),
        };

        let receipt = ExecReceipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(returns),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_exec_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
