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

use svm_types::{Gas, SpawnReceipt};

use super::{decode_error, encode_error, logs, returndata, TY_SPAWN};
use crate::{version, Codec};
use crate::{ReadExt, WriteExt};

impl Codec for SpawnReceipt {
    type Error = std::convert::Infallible;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(TY_SPAWN);
        encode_version(self, w);
        w.write_bool(self.success);

        if self.success {
            encode_account_addr(self, w);
            encode_init_state(self, w);
            encode_returndata(&self, w);
            self.gas_used().encode(w);
            logs::encode_logs(&self.logs, w);
        } else {
            let logs = self.logs();

            encode_error(self.error(), logs, w);
        };
    }

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let ty = cursor.read_byte().unwrap();
        debug_assert_eq!(ty, TY_SPAWN);

        let version = version::decode_version(cursor).unwrap();
        debug_assert_eq!(0, version);

        let is_success = cursor.read_bool().unwrap();

        match is_success {
            false => {
                let (err, logs) = decode_error(cursor);
                Ok(SpawnReceipt::from_err(err, logs))
            }
            true => {
                let addr = cursor.read_address().unwrap();
                let init_state = cursor.read_state().unwrap();
                let returndata = returndata::decode(cursor).unwrap();
                let gas_used = Gas::decode(cursor).unwrap();
                let logs = logs::decode_logs(cursor).unwrap();

                Ok(SpawnReceipt {
                    version,
                    success: true,
                    error: None,
                    account_addr: Some(addr.into()),
                    init_state: Some(init_state),
                    returndata: Some(returndata),
                    gas_used,
                    logs,
                })
            }
        }
    }
}

fn encode_version(receipt: &SpawnReceipt, w: &mut impl WriteExt) {
    let v = &receipt.version;
    version::encode_version(*v, w);
}

fn encode_account_addr(receipt: &SpawnReceipt, w: &mut impl WriteExt) {
    debug_assert!(receipt.success);

    let addr = receipt.account_addr();
    w.write_address(addr);
}

fn encode_init_state(receipt: &SpawnReceipt, w: &mut impl WriteExt) {
    debug_assert!(receipt.success);

    let state = receipt.init_state();
    w.write_state(state);
}

fn encode_returndata(receipt: &SpawnReceipt, w: &mut impl WriteExt) {
    debug_assert!(receipt.success);

    let data = receipt.returndata();
    returndata::encode(&data, w);
}

#[cfg(test)]
mod tests {
    use svm_types::{
        Address, BytesPrimitive, Gas, Receipt, ReceiptLog, RuntimeError, State, TemplateAddr,
    };

    use super::*;

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

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_spawn(), receipt);
    }

    #[test]
    fn encode_decode_spawn_receipt_success_without_returns() {
        let addr = Address::of("@Account").into();
        let init_state = State::of("some-state");
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

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

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_spawn(), receipt);
    }

    #[test]
    fn encode_decode_spawn_receipt_success_with_returns() {
        let addr = Address::of("@Account");
        let init_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

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

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_spawn(), receipt);
    }
}
