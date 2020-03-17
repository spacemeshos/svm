//!
//!           `Spawn App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +-------------------------------------------------------+
//!  |   format   |              |                           |
//!  |  version   |  is_success  |     App Address           |
//!  |  (4 bytes) |   (1 byte)   |      (20 bytes)           |
//!  +____________|______________|___________________________+
//!  |              |           |          |        |        |
//!  |  init state  | #returns  | ret #1   | ret #1 |        |
//!  |  (32 bytes)  | (2 bytes) |   type   |  value |  ...   |
//!  |              |           | (1 byte) |        |        |
//!  +______________|___________|__________|________|________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use byteorder::{BigEndian, WriteBytesExt};

use svm_common::Address;
use svm_runtime::{
    error::DeployTemplateError,
    receipt::{Receipt, SpawnAppReceipt},
};

use super::{encode_error, helpers};

pub(crate) fn encode_app_receipt(receipt: &SpawnAppReceipt) -> Vec<u8> {
    let mut buf = Vec::new();

    let wrapped_receipt = Receipt::SpawnApp(receipt);

    helpers::encode_is_success(&mut buf, &wrapped_receipt);

    if receipt.success {
        encode_app_addr(&mut buf, receipt);
        encode_init_state(&mut buf, receipt);
        helpers::encode_returns(&mut buf, &wrapped_receipt);
    } else {
        encode_error(&mut buf, &wrapped_receipt);
    };

    buf
}

fn encode_app_addr(buf: &mut Vec<u8>, receipt: &SpawnAppReceipt) {
    debug_assert!(receipt.success);

    let addr = receipt.get_app_addr();

    buf.extend_from_slice(addr.inner().as_slice());
}

fn encode_init_state(buf: &mut Vec<u8>, receipt: &SpawnAppReceipt) {
    debug_assert!(receipt.success);

    let state = receipt.get_init_state();

    buf.extend_from_slice(state.as_slice());
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testing::{self, ClientAppReceipt};

    use svm_app::types::AppAddr;
    use svm_common::{Address, State};
    use svm_runtime::{error::SpawnAppError, value::Value};

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
            gas_used: None,
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_app_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_app_receipt_success_without_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::from(0x10_20_30_40);

        let expected = ClientAppReceipt::Success {
            addr: addr.clone(),
            init_state: init_state.clone(),
            ctor_returns: "".to_string(),
        };

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returns: Some(Vec::new()),
            gas_used: Some(100),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_app_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn encode_decode_app_receipt_success_with_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::from(0x10_20_30_40);
        let returns = vec![Value::I32(10), Value::I64(20), Value::I32(30)];

        let expected = ClientAppReceipt::Success {
            addr: addr.clone(),
            init_state: init_state.clone(),
            ctor_returns: "I32(10), I64(20), I32(30)".to_string(),
        };

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returns: Some(returns),
            gas_used: Some(100),
        };

        let bytes = encode_app_receipt(&receipt);
        let actual = testing::decode_app_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
