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

use std::convert::TryFrom;

use svm_types::{CallReceipt, Gas, ReceiptLog};

use super::error::RuntimeErrorWithLogs;
use super::returndata;
use crate::{Codec, ReadExt, WriteExt};

impl Codec for CallReceipt {
    type Error = std::convert::Infallible;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(super::TY_CALL);
        u16::try_from(self.version).unwrap().encode(w);
        self.success.encode(w);

        if self.success {
            self.new_state().0.encode(w);
            returndata::encode(&self.returndata(), w);
            self.gas_used.encode(w);
            self.logs.encode(w);
        } else {
            let logs = self.logs();
            RuntimeErrorWithLogs::new(self.error().clone(), logs).encode(w);
        };
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let ty = reader.read_byte().unwrap();
        debug_assert_eq!(ty, crate::receipt::TY_CALL);

        let version = u16::decode(reader).unwrap();
        debug_assert_eq!(0, version);

        let is_success = bool::decode(reader).unwrap();

        if is_success {
            let new_state = <[u8; 32]>::decode(reader).unwrap().into();
            let returndata = returndata::decode(reader).unwrap();
            let gas_used = Gas::decode(reader).unwrap();
            let logs = <Vec<ReceiptLog>>::decode(reader).unwrap();

            Ok(CallReceipt {
                version,
                success: true,
                error: None,
                new_state: Some(new_state),
                returndata: Some(returndata),
                gas_used,
                logs,
            })
        } else {
            let x = RuntimeErrorWithLogs::decode(reader).unwrap();
            Ok(CallReceipt::from_err(x.err, x.logs))
        }
    }
}

#[cfg(test)]
mod tests {
    use svm_types::{Address, BytesPrimitive, Gas, Receipt, ReceiptLog, RuntimeError, State};

    use super::*;

    #[test]
    fn encode_decode_call_receipt_error() {
        let account = Address::of("@Account");
        let error = RuntimeError::AccountNotFound(account.into());

        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        let receipt = CallReceipt {
            version: 0,
            success: false,
            error: Some(error),
            new_state: None,
            returndata: None,
            gas_used: Gas::new(),
            logs,
        };

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_call(), receipt);
    }

    #[test]
    fn encode_decode_call_receipt_success_without_returns() {
        let new_state = State::of("some-state");
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        let receipt = CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(Vec::new()),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_call(), receipt);
    }

    #[test]
    fn encode_decode_call_receipt_success_with_returns() {
        let new_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];

        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        let receipt = CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(returndata),
            gas_used: Gas::with(100),
            logs: logs.clone(),
        };

        let bytes = receipt.encode_to_vec();
        let decoded = Receipt::decode_bytes(bytes).unwrap();

        assert_eq!(decoded.into_call(), receipt);
    }
}
