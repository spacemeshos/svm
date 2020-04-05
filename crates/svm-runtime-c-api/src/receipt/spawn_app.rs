//!           `Spawn App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +----------------------------------------------------+
//!  |   format   |              |                        |
//!  |  version   |  is_success  |     App Address        |
//!  |  (4 bytes) |   (1 byte)   |      (20 bytes)        |
//!  +____________|______________|________________________+
//!  |              |           |             |           |
//!  |  init state  | #returns  | ret #1 type | ret  #1   |
//!  |  (32 bytes)  |           |             |           |     
//!  +______________|___________|_____________|___________+
//!  |          |            |                            |
//!  |  ret #2  |   .  .  .  |         gas_used           |
//!  +__________|____________|____________________________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use svm_app::raw::NibbleWriter;
use svm_runtime::receipt::{Receipt, SpawnAppReceipt};

use super::{encode_error, helpers};

pub(crate) fn encode_app_receipt(receipt: &SpawnAppReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::SpawnApp(receipt);

    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_app_addr(receipt, &mut w);
        encode_init_state(receipt, &mut w);
        helpers::encode_returns(&wrapped_receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testing::{self, ClientAppReceipt};

    use svm_app::{types::AppAddr, types::WasmValue};
    use svm_common::{Address, State};
    use svm_runtime::{error::SpawnAppError, gas::MaybeGas};

    #[test]
    fn encode_decode_app_receipt_error() {
        let template_addr = Address::of("my-template").into();

        let error = SpawnAppError::TemplateNotFound(template_addr);

        let expected = ClientAppReceipt::Failure {
            error: error.to_string(),
        };

        let receipt = SpawnAppReceipt {
            success: false,
            error: Some(error),
            app_addr: None,
            init_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_app_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_app_receipt_success_without_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::of("some-state");

        let expected = ClientAppReceipt::Success {
            addr: addr.clone(),
            init_state: init_state.clone(),
            ctor_returns: "".to_string(),
            gas_used: 100,
        };

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returns: Some(Vec::new()),
            gas_used: MaybeGas::with(100),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_app_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_app_receipt_success_with_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::of("some-state");
        let returns = vec![WasmValue::I32(10), WasmValue::I64(20), WasmValue::I32(30)];

        let expected = ClientAppReceipt::Success {
            addr: addr.clone(),
            init_state: init_state.clone(),
            ctor_returns: "I32(10), I64(20), I32(30)".to_string(),
            gas_used: 100,
        };

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returns: Some(returns),
            gas_used: MaybeGas::with(100),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_app_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
