//! ## `Spawn Account` Receipt Binary Format Version 0
//!
//! On success (`is_success = 1`)
//!
//! ```text
//! +---------------------------------------------------------+
//! |           |            |             |                  |
//! |  tx type  |  version   | is_success  |  Account Address |
//! | (1 byte)  | (2 bytes)  |  (1 byte)   |    (20 bytes)    |
//! |           |            |             |                  |
//! +---------------------------------------------------------+
//! |              |              |              |            |
//! |  init State  | returndata   |  returndata  |  gas_used  |
//! |  (32 bytes)  |  byte-size   |   (Blob)     | (8 bytes)  |
//! |              |  (2 bytes)   |              |            |
//! |              |              |              |            |
//! +---------------------------------------------------------+
//! |           |              |           |                  |
//! |  #touched |    t.a. #1   |   . . .   |     t.a. #N      |
//! |  accounts |              |           |                  |
//! | (2 bytes) |  (20 bytes)  |           |   (20 bytes)     |
//! |           |              |           |                  |
//! +---------------------------------------------------------+
//! |           |          |         |                        |
//! |  #logs    |  log #1  |  . . .  |       log #N           |
//! | (1 byte)  |  (Blob)  |         |       (Blob)           |
//! |           |          |         |                        |
//! +---------------------------------------------------------+
//! ```
//!
//!
//! On Error (`is_success = 0`)
//! See [error.rs][./error.rs]

use std::collections::HashSet;

use svm_types::{Address, Gas, ReceiptLog, RuntimeFailure, SpawnReceipt, State};

use super::TY_SPAWN;
use crate::{codec::ReturnData, Codec, ParseError, ReadExt, WriteExt};

impl Codec for SpawnReceipt {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(TY_SPAWN);
        self.version.encode(w);
        self.success.encode(w);

        if self.success {
            self.account_addr().encode(w);
            self.init_state().encode(w);
            ReturnData::new(self.returndata().clone()).encode(w);
            self.gas_used().encode(w);
            self.touched_accounts.encode(w);
            self.logs.encode(w);
        } else {
            RuntimeFailure::new(self.error().clone(), self.logs().clone()).encode(w);
        };
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let ty = reader.read_byte()?;
        debug_assert_eq!(ty, TY_SPAWN);

        let version = u16::decode(reader)?;
        debug_assert_eq!(0, version);

        let is_success = bool::decode(reader)?;

        if is_success {
            let addr = Address::decode(reader)?;
            let init_state = State::decode(reader)?;
            let returndata = ReturnData::decode(reader)?.data;
            let gas_used = Gas::decode(reader)?;
            let touched_accounts = <HashSet<Address>>::decode(reader)?;
            let logs = <Vec<ReceiptLog>>::decode(reader)?;

            Ok(SpawnReceipt {
                version,
                success: true,
                error: None,
                account_addr: Some(addr.into()),
                init_state: Some(init_state),
                returndata: Some(returndata),
                gas_used,
                touched_accounts,
                logs,
            })
        } else {
            let x = RuntimeFailure::decode(reader).unwrap();
            Ok(SpawnReceipt::from_err(x.err, x.logs))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use svm_types::{Address, BytesPrimitive, Gas, ReceiptLog, RuntimeError, State, TemplateAddr};

    use crate::codec::test_codec;

    use super::*;

    #[test]
    fn encode_decode_spawn_receipt_error() {
        let template_addr = TemplateAddr::of("@Template");
        let error = RuntimeError::TemplateNotFound(template_addr);

        test_codec(SpawnReceipt {
            version: 0,
            success: false,
            error: Some(error),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            touched_accounts: HashSet::new(),
            logs: Vec::new(),
        });
    }

    #[test]
    fn encode_decode_spawn_receipt_success_without_returns() {
        let addr = Address::of("@Account").into();
        let init_state = State::of("some-state");
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        test_codec(SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(Vec::new()),
            gas_used: Gas::with(100),
            touched_accounts: HashSet::new(),
            logs: logs.clone(),
        });
    }

    #[test]
    fn encode_decode_spawn_receipt_success_with_returns() {
        let addr = Address::of("@Account");
        let init_state = State::of("some-state");
        let returndata = vec![0x10, 0x20];
        let mut touched_accounts = HashSet::new();
        touched_accounts.insert(addr.clone());
        touched_accounts.insert(Address::repeat(0xff));
        touched_accounts.insert(Address::zeros());
        let logs = vec![ReceiptLog::new(b"something happened".to_vec())];

        test_codec(SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(addr),
            init_state: Some(init_state),
            returndata: Some(returndata),
            gas_used: Gas::with(100),
            touched_accounts,
            logs: logs.clone(),
        });
    }
}
