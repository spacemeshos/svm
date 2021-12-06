//! ## `Call Account` Receipt Binary Format Version 0
//!
//! On success (`is_success = 1`)
//!
//! ```text
//! +---------------------------------------------------+
//! |           |            |            |             |
//! |  tx type  |  version   | is_success |  new State  |
//! | (1 byte)  |  (2 bytes) |  (1 byte)  | (32 bytes)  |
//! |           |            |            |             |
//! +---------------------------------------------------+
//! |              |             |                      |
//! |  returndata  | returndata  |      gas_used        |
//! |   byte-size  |   (Blob)    |      (8 bytes)       |
//! |   (2 bytes)  |             |                      |
//! |              |             |                      |
//! +---------------------------------------------------+
//! |           |            |       |                  |
//! |  #touched |   t.a. #1  | . . . |     t.a. #N      |
//! |  accounts |            |       |                  |
//! | (2 bytes) | (20 bytes) |       |   (20 bytes)     |
//! |           |            |       |                  |
//! +---------------------------------------------------+
//! |           |          |         |                  |
//! |  #logs    |  log #1  |  . . .  |     log #N       |
//! | (1 byte)  |  (Blob)  |         |     (Blob)       |
//! |           |          |         |                  |
//! +---------------------------------------------------+
//! ```
//!
//!
//! On Error (`is_success = 0`)
//! See [error.rs](./error.rs)

use std::{collections::HashSet, convert::TryFrom};

use svm_types::{Address, CallReceipt, Gas, ReceiptLog, RuntimeFailure, State};

use crate::{codec::ReturnData, Codec, ParseError, ReadExt, WriteExt};

impl Codec for CallReceipt {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(super::TY_CALL);
        u16::try_from(self.version).unwrap().encode(w);
        self.success.encode(w);

        if self.success {
            self.new_state().0.encode(w);
            ReturnData::new(self.returndata().clone()).encode(w);
            self.gas_used.encode(w);
            self.touched_accounts.encode(w);
            self.logs.encode(w);
        } else {
            let logs = self.logs();
            RuntimeFailure::new(self.error().clone(), logs).encode(w);
        };
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let ty = reader.read_byte()?;
        debug_assert_eq!(ty, crate::receipt::TY_CALL);

        let version = u16::decode(reader)?;
        debug_assert_eq!(0, version);

        let is_success = bool::decode(reader)?;

        if is_success {
            let new_state = State::decode(reader)?;
            let returndata = ReturnData::decode(reader)?.data;
            let gas_used = Gas::decode(reader)?;
            let touched_accounts = <HashSet<Address>>::decode(reader)?;
            let logs = <Vec<ReceiptLog>>::decode(reader)?;

            Ok(CallReceipt {
                version,
                success: true,
                error: None,
                new_state: Some(new_state),
                returndata: Some(returndata),
                gas_used,
                touched_accounts,
                logs,
            })
        } else {
            let x = RuntimeFailure::decode(reader)?;
            Ok(CallReceipt::from_err(x.err, x.logs))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use svm_types::{Address, BytesPrimitive, Gas, ReceiptLog, RuntimeError, State};

    use crate::codec::test_codec;

    use super::*;

    #[test]
    fn encode_decode_call_receipt_error() {
        let account = Address::of("@Account");
        let error = RuntimeError::AccountNotFound(account.into());
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        test_codec(CallReceipt {
            version: 0,
            success: false,
            error: Some(error),
            new_state: None,
            returndata: None,
            gas_used: Gas::new(),
            touched_accounts: HashSet::new(),
            logs,
        });
    }

    #[test]
    fn encode_decode_call_receipt_success_without_returns() {
        let new_state = State::of("some-state");
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        test_codec(CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(Vec::new()),
            gas_used: Gas::with(100),
            touched_accounts: HashSet::new(),
            logs: logs.clone(),
        });
    }

    #[test]
    fn encode_decode_call_receipt_success_with_returns() {
        let new_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        test_codec(CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(new_state),
            returndata: Some(returndata),
            gas_used: Gas::with(100),
            touched_accounts: HashSet::new(),
            logs: logs.clone(),
        });
    }
}
