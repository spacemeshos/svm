//! `Spawn App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +-----------------------------------------------------+
//!  |  tx type  |  version   | is_success  | App Address  |
//!  | (1 byte)  |            |  (1 byte)   |  (20 bytes)  |
//!  +___________|____________|_____________|______________+
//!  |              |              |                       |
//!  |  init state  | #returndata  |      gas_used         |
//!  |  (32 bytes)  |              |                       |
//!  +______________|______________|_______________________+
//!  |          |            |         |                   |
//!  |  #logs   | log 1 blob |  . . .  |     log #N        |
//!  +__________|____________|_________|___________________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use std::io::Cursor;

use svm_types::SpawnAppReceipt;

use super::{decode_error, encode_error, gas, logs, types};

use crate::{calldata, common};
use crate::{ReadExt, WriteExt};

/// Encodes a `spawn-app` receipt into its binary format.
pub fn encode_app_receipt(receipt: &SpawnAppReceipt) -> Vec<u8> {
    let mut w = Vec::new();

    w.write_byte(types::SPAWN_APP);
    encode_version(receipt, &mut w);
    w.write_bool(receipt.success);

    if receipt.success {
        encode_app_addr(receipt, &mut w);
        encode_init_state(receipt, &mut w);
        encode_returndata(&receipt, &mut w);
        gas::encode_gas_used(&receipt.gas_used, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = receipt.get_logs();

        encode_error(receipt.get_error(), logs, &mut w);
    };

    w
}

/// Decodes a binary `spawn-app` receipt into its Rust struct.
pub fn decode_app_receipt(bytes: &[u8]) -> SpawnAppReceipt {
    let mut cursor = Cursor::new(bytes);

    let ty = cursor.read_byte().unwrap();
    debug_assert_eq!(ty, types::SPAWN_APP);

    let version = common::decode_version(&mut cursor).unwrap();
    debug_assert_eq!(0, version);

    let is_success = cursor.read_bool().unwrap();

    match is_success {
        false => {
            let (err, logs) = decode_error(&mut cursor);
            SpawnAppReceipt::from_err(err, logs)
        }
        true => {
            let addr = cursor.read_address().unwrap();
            let init_state = cursor.read_state().unwrap();
            let returndata = calldata::decode_calldata(&mut cursor).unwrap();
            let gas_used = gas::decode_gas_used(&mut cursor).unwrap();
            let logs = logs::decode_logs(&mut cursor).unwrap();

            SpawnAppReceipt {
                version,
                success: true,
                error: None,
                app_addr: Some(addr.into()),
                init_state: Some(init_state),
                returndata: Some(returndata),
                gas_used,
                logs,
            }
        }
    }
}

fn encode_version(receipt: &SpawnAppReceipt, w: &mut Vec<u8>) {
    let v = &receipt.version;

    common::encode_version(*v, w);
}

fn encode_app_addr(receipt: &SpawnAppReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let addr = receipt.get_app_addr();

    w.write_address(addr.inner());
}

fn encode_init_state(receipt: &SpawnAppReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let state = receipt.get_init_state();

    w.write_state(state);
}

fn encode_returndata(receipt: &SpawnAppReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let data = receipt.get_returndata();

    calldata::encode_calldata(&data, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::{Address, AppAddr, Gas, Log, RuntimeError, State};

    use crate::receipt::decode_receipt;

    #[test]
    fn encode_decode_spawn_app_receipt_error() {
        let template_addr = Address::of("my-template").into();

        let error = RuntimeError::TemplateNotFound(template_addr);

        let receipt = SpawnAppReceipt {
            version: 0,
            success: false,
            error: Some(error),
            app_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            logs: Vec::new(),
        };

        let bytes = encode_app_receipt(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn_app(), receipt);
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
            version: 0,
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(Vec::new()),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_app_receipt(&receipt);
        let decoded = decode_receipt(&bytes);

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
            version: 0,
            success: true,
            error: None,
            app_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(returndata),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_app_receipt(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn_app(), receipt);
    }
}
