//!           `Spawn App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +-----------------------------------------------------+
//!  |   tx type  |  version   | is_success |  App Address |
//!  | (1 byte)   | (1 nibble) | (1 nibble) |  (20 bytes)  |
//!  +____________|____________|____________|______________+
//!  |              |           |             |            |
//!  |  init state  | #returns  | ret #1 type | ret  #1    |
//!  |  (32 bytes)  |           |             |            |
//!  +______________|___________|_____________|____________+
//!  |          |            |                             |
//!  |  ret #2  |   .  .  .  |         gas_used            |
//!  +__________|____________|_____________________________+
//!  |          |            |         |                   |
//!  |  #logs   | log 1 blob |  . . .  |     log #N        |
//!  +__________|____________|_________|___________________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use crate::nibble::NibbleWriter;
use svm_types::receipt::{Receipt, SpawnAppReceipt};

use super::{encode_error, helpers, logs::encode_logs};

pub fn encode_app_receipt(receipt: &SpawnAppReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::SpawnApp(receipt);

    helpers::encode_type(super::types::SPAWN_APP, &mut w);
    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_app_addr(receipt, &mut w);
        encode_init_state(receipt, &mut w);
        encode_returns(&receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
        encode_logs(&receipt.logs, &mut w);
    } else {
        encode_error(&wrapped_receipt, &mut w);
    };

    w.into_bytes()
}

fn encode_app_addr(receipt: &SpawnAppReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let addr = receipt.get_app_addr();
    helpers::encode_addr(addr.inner(), w)
}

fn encode_init_state(receipt: &SpawnAppReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let state = receipt.get_init_state();
    helpers::encode_state(&state, w);
}

fn encode_returns(receipt: &SpawnAppReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let returns = receipt.get_returns();
    helpers::encode_returns(&returns, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::receipt::testing::{self, ClientAppReceipt, ClientReceipt};

    use svm_types::receipt::{error::SpawnAppError, Log};
    use svm_types::{gas::MaybeGas, Address, AppAddr, State, WasmValue};

    #[test]
    fn encode_decode_app_receipt_error() {
        let template_addr = Address::of("my-template").into();

        let error = SpawnAppError::TemplateNotFound(template_addr);

        let expected = ClientReceipt::SpawnApp(ClientAppReceipt::Failure {
            error: error.to_string(),
        });

        let receipt = SpawnAppReceipt {
            success: false,
            error: Some(error),
            app_addr: None,
            init_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
            logs: Vec::new(),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_app_receipt_success_without_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::of("some-state");

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let expected = ClientReceipt::SpawnApp(ClientAppReceipt::Success {
            addr: addr.clone(),
            init_state: init_state.clone(),
            ctor_returns: Vec::new(),
            gas_used: 100,
            logs: logs.clone(),
        });

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returns: Some(Vec::new()),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_app_receipt_success_with_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::of("some-state");
        let returns = vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)];
        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let expected = ClientReceipt::SpawnApp(ClientAppReceipt::Success {
            addr: addr.clone(),
            init_state: init_state.clone(),
            ctor_returns: vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)],
            gas_used: 100,
            logs: logs.clone(),
        });

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returns: Some(returns),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
