//!  ## `Call Account` Receipt Binary Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  ```text
//!  +---------------------------------------------------+
//!  |           |            |            |             |
//!  |  tx type  |  version   | is_success |  new State  |
//!  | (1 byte)  |  (2 bytes) |  (1 byte)  | (32 bytes)  |
//!  |           |            |            |             |
//!  +---------------------------------------------------+
//!  |              |             |                      |
//!  |  returndata  | returndata  |      gas_used        |
//!  |   byte-size  |   (Blob)    |      (8 bytes)       |
//!  |   (2 bytes)  |             |                      |
//!  |              |             |                      |
//!  +---------------------------------------------------+
//!  |           |          |         |                  |
//!  |  #logs    |  log #1  |  . . .  |     log #N       |
//!  | (1 byte)  |  (Blob)  |         |     (Blob)       |
//!  |           |          |         |                  |
//!  +---------------------------------------------------+
//!  ```
//!
//!
//!  On Error (`is_success = 0`)
//!  See [error.rs](./error.rs)

use std::io::Cursor;

use svm_types::CallReceipt;

use super::{decode_error, encode_error, gas, logs, returndata};
use crate::version;
use crate::{ReadExt, WriteExt};

/// Encodes an [`CallReceipt`] into its binary format.
pub fn encode_call(receipt: &CallReceipt) -> Vec<u8> {
    let mut w = Vec::new();

    w.write_byte(super::types::CALL);
    version::encode_version(receipt.version, &mut w);
    w.write_bool(receipt.success);

    if receipt.success {
        encode_new_state(receipt, &mut w);
        encode_returndata(receipt, &mut w);
        gas::encode_gas_used(&receipt.gas_used, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = receipt.logs();

        encode_error(receipt.error(), logs, &mut w);
    };

    w
}

/// Decodes a binary [`CallReceipt`].
pub fn decode_call(bytes: &[u8]) -> CallReceipt {
    let mut cursor = Cursor::new(bytes);

    let ty = cursor.read_byte().unwrap();
    debug_assert_eq!(ty, crate::receipt::types::CALL);

    let version = version::decode_version(&mut cursor).unwrap();
    debug_assert_eq!(0, version);

    let is_success = cursor.read_bool().unwrap();

    match is_success {
        false => {
            let (err, logs) = decode_error(&mut cursor);
            CallReceipt::from_err(err, logs)
        }
        true => {
            let new_state = cursor.read_state().unwrap();
            let returndata = returndata::decode(&mut cursor).unwrap();
            let gas_used = gas::decode_gas_used(&mut cursor).unwrap();
            let logs = logs::decode_logs(&mut cursor).unwrap();

            CallReceipt {
                version,
                success: true,
                error: None,
                new_state: Some(new_state),
                returndata: Some(returndata),
                gas_used,
                logs,
            }
        }
    }
}

fn encode_new_state(receipt: &CallReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let state = receipt.new_state();
    w.write_state(state);
}

fn encode_returndata(receipt: &CallReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let data = receipt.returndata();
    returndata::encode(&data, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::{Address, Gas, ReceiptLog, RuntimeError, State};

    #[test]
    fn encode_decode_call_receipt_error() {
        let account = Address::of("@Account");
        let error = RuntimeError::AccountNotFound(account.into());

        let logs = vec![ReceiptLog {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = CallReceipt {
            version: 0,
            success: false,
            error: Some(error),
            new_state: None,
            returndata: None,
            gas_used: Gas::new(),
            logs,
        };

        let bytes = encode_call(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_call(), receipt);
    }

    #[test]
    fn encode_decode_call_receipt_success_without_returns() {
        let new_state = State::of("some-state");

        let logs = vec![ReceiptLog {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(Vec::new()),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_call(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_call(), receipt);
    }

    #[test]
    fn encode_decode_call_receipt_success_with_returns() {
        let new_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];

        let logs = vec![ReceiptLog {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(returndata),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_call(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes[..]);

        assert_eq!(decoded.into_call(), receipt);
    }
}
