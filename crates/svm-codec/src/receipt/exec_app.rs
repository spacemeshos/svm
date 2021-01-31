//! `Exec App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +---------------------------------------------------+
//!  |  tx type  |  version   | is_success |  new state  |
//!  | (1 byte)  |  (2 bytes) |  (1 byte)  | (32 bytes)  |
//!  +___________|____________|____________|_____________+
//!  |                       |                           |
//!  |       returndata      |    gas_used (8 bytes)     |
//!  +_______________________|___________________________+
//!  |          |            |         |                 |
//!  |  #logs   | log 1 blob |  . . .  |     log #N      |
//!  +__________|____________|_________|_________________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use std::io::Cursor;

use svm_types::gas::MaybeGas;
use svm_types::receipt::{ExecReceipt, Log, Receipt};

use super::{decode_error, encode_error, gas, logs};

use crate::{calldata, common};
use crate::{ReadExt, WriteExt};

pub fn encode_exec_receipt(receipt: &ExecReceipt) -> Vec<u8> {
    let mut w = Vec::new();

    w.write_byte(super::types::EXEC_APP);
    common::encode_version(receipt.version, &mut w);
    w.write_bool(receipt.success);

    if receipt.success {
        encode_new_state(receipt, &mut w);
        encode_returndata(receipt, &mut w);
        gas::encode_gas_used(&receipt.gas_used, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = receipt.get_logs();

        encode_error(receipt.get_error(), logs, &mut w);
    };

    w
}

pub fn decode_exec_receipt(bytes: &[u8]) -> ExecReceipt {
    let mut cursor = Cursor::new(bytes);

    let ty = cursor.read_byte().unwrap();
    debug_assert_eq!(ty, crate::receipt::types::EXEC_APP);

    let version = common::decode_version(&mut cursor).unwrap();
    debug_assert_eq!(0, version);

    let is_success = cursor.read_bool().unwrap();

    match is_success {
        false => {
            let (err, logs) = decode_error(&mut cursor);
            ExecReceipt::from_err(err, logs)
        }
        true => {
            let new_state = cursor.read_state().unwrap();
            let returndata = calldata::decode_calldata(&mut cursor).unwrap();
            let gas_used = gas::decode_gas_used(&mut cursor).unwrap();
            let logs = logs::decode_logs(&mut cursor).unwrap();

            ExecReceipt {
                version,
                success: true,
                error: None,
                new_state: Some(new_state),
                returndata: Some(returndata),
                gas_used,
                logs,
            }
        }
        _ => unreachable!(),
    }
}

fn encode_new_state(receipt: &ExecReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let state = receipt.get_new_state();
    w.write_state(state);
}

fn encode_returndata(receipt: &ExecReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let data = receipt.get_returndata();
    calldata::encode_calldata(&data, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::gas::MaybeGas;
    use svm_types::receipt::ReceiptError;
    use svm_types::{Address, State};

    #[test]
    fn encode_decode_exec_receipt_error() {
        let app = Address::of("my-app");
        let error = ReceiptError::AppNotFound(app.into());

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = ExecReceipt {
            version: 0,
            success: false,
            error: Some(error),
            new_state: None,
            returndata: None,
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
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(Vec::new()),
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
        let returndata = vec![0x10, 0x20];

        let logs = vec![Log {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = ExecReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(returndata),
            gas_used: MaybeGas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_exec_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_exec_app(), receipt);
    }
}
