//!  ## `Spawn Account` Receipt Binary Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  ```text
//!  +---------------------------------------------------------+
//!  |           |            |             |                  |
//!  |  tx type  |  version   | is_success  |  Account Address |
//!  | (1 byte)  | (2 bytes)  |  (1 byte)   |    (20 bytes)    |
//!  |           |            |             |                  |
//!  +---------------------------------------------------------+
//!  |              |              |              |            |
//!  |  init State  | returndata   |  returndata  |  gas_used  |
//!  |  (32 bytes)  |  byte-size   |   (Blob)     | (8 bytes)  |
//!  |              |  (2 bytes)   |              |            |
//!  |              |              |              |            |
//!  +---------------------------------------------------------+
//!  |           |          |         |                        |
//!  |  #logs    |  log #1  |  . . .  |       log #N           |
//!  | (1 byte)  |  (Blob)  |         |       (Blob)           |
//!  |           |          |         |                        |
//!  +---------------------------------------------------------+
//!  ```
//!
//!
//!  On Error (`is_success = 0`)
//!  See [error.rs][./error.rs]

use svm_types::SpawnReceipt;

use std::io::Cursor;

use super::{decode_error, encode_error, gas, logs, returndata, types};
use crate::version;
use crate::{ReadExt, WriteExt};

/// Encodes a [`SpawnReceipt`] into its binary format.
pub fn encode_spawn(receipt: &SpawnReceipt) -> Vec<u8> {
    let mut w = Vec::new();

    w.write_byte(types::SPAWN);
    encode_version(receipt, &mut w);
    w.write_bool(receipt.success);

    if receipt.success {
        encode_account_addr(receipt, &mut w);
        encode_init_state(receipt, &mut w);
        encode_returndata(&receipt, &mut w);
        gas::encode_gas_used(&receipt.gas_used, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = receipt.logs();

        encode_error(receipt.error(), logs, &mut w);
    };

    w
}

/// Decodes a binary [`SpawnReceipt`].
pub fn decode_spawn(bytes: &[u8]) -> SpawnReceipt {
    let mut cursor = Cursor::new(bytes);

    let ty = cursor.read_byte().unwrap();
    debug_assert_eq!(ty, types::SPAWN);

    let version = version::decode_version(&mut cursor).unwrap();
    debug_assert_eq!(0, version);

    let is_success = cursor.read_bool().unwrap();

    match is_success {
        false => {
            let (err, logs) = decode_error(&mut cursor);
            SpawnReceipt::from_err(err, logs)
        }
        true => {
            let addr = cursor.read_address().unwrap();
            let init_state = cursor.read_state().unwrap();
            let returndata = returndata::decode(&mut cursor).unwrap();
            let gas_used = gas::decode_gas_used(&mut cursor).unwrap();
            let logs = logs::decode_logs(&mut cursor).unwrap();

            SpawnReceipt {
                version,
                success: true,
                error: None,
                account_addr: Some(addr.into()),
                init_state: Some(init_state),
                returndata: Some(returndata),
                gas_used,
                logs,
            }
        }
    }
}

fn encode_version(receipt: &SpawnReceipt, w: &mut Vec<u8>) {
    let v = &receipt.version;
    version::encode_version(*v, w);
}

fn encode_account_addr(receipt: &SpawnReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let addr = receipt.account_addr();
    w.write_address(addr);
}

fn encode_init_state(receipt: &SpawnReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let state = receipt.init_state();
    w.write_state(state);
}

fn encode_returndata(receipt: &SpawnReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let data = receipt.returndata();
    returndata::encode(&data, w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::{Address, Gas, ReceiptLog, RuntimeError, State, TemplateAddr};

    use crate::receipt::decode_receipt;

    #[test]
    fn encode_decode_spawn_receipt_error() {
        let template_addr = TemplateAddr::of("@Template");
        let error = RuntimeError::TemplateNotFound(template_addr);

        let receipt = SpawnReceipt {
            version: 0,
            success: false,
            error: Some(error),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            logs: Vec::new(),
        };

        let bytes = encode_spawn(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn(), receipt);
    }

    #[test]
    fn encode_decode_spawn_receipt_success_without_returns() {
        let addr = Address::of("@Account").into();
        let init_state = State::of("some-state");

        let logs = vec![ReceiptLog {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(Vec::new()),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_spawn(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn(), receipt);
    }

    #[test]
    fn encode_decode_spawn_receipt_success_with_returns() {
        let addr = Address::of("@Account");
        let init_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];
        let logs = vec![ReceiptLog {
            msg: b"something happened".to_vec(),
            code: 200,
        }];

        let receipt = SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(returndata),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = encode_spawn(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_spawn(), receipt);
    }
}
