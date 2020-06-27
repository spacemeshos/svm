//!         `ExecReceipt` Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +---------------------------------------------------+
//!  |            |              |                       |
//!  |  version   |  is_success  |     app new state     |
//!  |            |  (1 nibble)  |      (32 bytes)       |
//!  +____________|______________|_______________________+
//!  |          |              |         |               |
//!  | #returns | ret #1 type  | ret #1  |  ret #2  type |
//!  +__________|______________|_________|_______________+
//!  |          |            |                           |
//!  |  ret #2  |   .  .  .  |         gas_used          |
//!  +__________|____________|___________________________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use svm_codec::nibble::NibbleWriter;
use svm_runtime::receipt::{ExecReceipt, Receipt};

use super::{encode_error, helpers};

pub(crate) fn encode_exec_receipt(receipt: &ExecReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::ExecApp(receipt);

    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_new_state(receipt, &mut w);
        encode_returns(receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
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

    use crate::testing::{self, ClientExecReceipt};

    use svm_app::types::WasmValue;
    use svm_common::{Address, State};
    use svm_runtime::{error::ExecAppError, gas::MaybeGas};

    #[test]
    fn encode_decode_exec_receipt_error() {
        let error = ExecAppError::AppNotFound {
            app_addr: Address::of("my-app").into(),
        };

        let expected = ClientExecReceipt::Failure {
            error: error.to_string(),
        };

        let receipt = ExecReceipt {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_exec_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_exec_receipt_success_without_returns() {
        let new_state = State::of("some-state");

        let expected = ClientExecReceipt::Success {
            new_state: new_state.clone(),
            func_returns: Vec::new(),
            gas_used: 100,
        };

        let receipt = ExecReceipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(Vec::new()),
            gas_used: MaybeGas::with(100),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_exec_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_exec_receipt_success_with_returns() {
        let new_state = State::of("some-state");
        let returns = vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)];

        let expected = ClientExecReceipt::Success {
            new_state: new_state.clone(),
            func_returns: vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)],
            gas_used: 100,
        };

        let receipt = ExecReceipt {
            success: true,
            error: None,
            new_state: Some(new_state),
            returns: Some(returns),
            gas_used: MaybeGas::with(100),
        };

        let bytes = encode_exec_receipt(&receipt);
        let actual = testing::decode_exec_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
