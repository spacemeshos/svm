//! `Spawn App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +-----------------------------------------------------+
//!  |  tx type  |  version   | is_success |  App Address  |
//!  | (1 byte)  | (1 nibble) | (1 nibble) |  (20 bytes)   |
//!  +___________|____________|____________|_______________+
//!  |              |           |             |            |
//!  |  init state  | #returndata  | ret #1 type |  ret  #1   |
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

use svm_nibble::{NibbleIter, NibbleWriter};

use crate::api::raw;

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Receipt, SpawnAppReceipt};

use super::{decode_error, encode_error, helpers, logs};

pub fn encode_app_receipt(receipt: &SpawnAppReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::SpawnApp(receipt);

    helpers::encode_type(super::types::SPAWN_APP, &mut w);
    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_app_addr(receipt, &mut w);
        encode_init_state(receipt, &mut w);
        encode_returndata(&receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = receipt.get_logs();

        encode_error(receipt.get_error(), logs, &mut w);
    };

    w.into_bytes()
}

pub fn decode_app_receipt(bytes: &[u8]) -> SpawnAppReceipt {
    let mut iter = NibbleIter::new(bytes);

    let ty = helpers::decode_type(&mut iter);
    debug_assert_eq!(ty, crate::receipt::types::SPAWN_APP);

    let version = helpers::decode_version(&mut iter).unwrap();
    debug_assert_eq!(0, version);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            let (err, logs) = decode_error(&mut iter);
            SpawnAppReceipt::from_err(err, logs)
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut iter);
            let init_state = helpers::decode_state(&mut iter);
            let returndata = raw::decode_abi_data(&mut iter).unwrap();
            let gas_used = helpers::decode_gas_used(&mut iter);
            let logs = logs::decode_logs(&mut iter);

            SpawnAppReceipt {
                success: true,
                error: None,
                app_addr: Some(addr.into()),
                init_state: Some(init_state),
                returndata: Some(returndata),
                gas_used,
                logs,
            }
        }
        _ => unreachable!(),
    }
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

fn encode_returndata(receipt: &SpawnAppReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let data = receipt.get_returndata();
    helpers::encode_abi_data(&data, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::receipt::{Log, ReceiptError};
    use svm_types::{gas::MaybeGas, Address, AppAddr, State};

    #[test]
    fn encode_decode_spawn_app_receipt_error() {
        let template_addr = Address::of("my-template").into();

        let error = ReceiptError::TemplateNotFound(template_addr);

        let receipt = SpawnAppReceipt {
            success: false,
            error: Some(error),
            app_addr: None,
            init_state: None,
            returndata: None,
            gas_used: MaybeGas::new(),
            logs: Vec::new(),
        };

        let bytes = encode_app_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes);

        // assert_eq!(decoded.into_spawn_app(), receipt);
    }

    #[test]
    fn encode_decode_spawn_app_receipt_success_without_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::of("some-state");

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(Vec::new()),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_app_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn_app(), receipt);
    }

    #[test]
    fn encode_decode_spawn_app_receipt_success_with_returns() {
        let addr: AppAddr = Address::of("my-app").into();
        let init_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];
        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(returndata),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_app_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn_app(), receipt);
    }
}
