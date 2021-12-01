//! Encoding of receipts.

mod call;
mod deploy;
mod error;
mod spawn;
mod touched_accounts;

pub(crate) mod logs;

use svm_types::{CallReceipt, DeployReceipt, Receipt, SpawnReceipt};

use crate::{Codec, ParseError, ReadExt};

impl Codec for Receipt {
    type Error = ParseError;

    fn encode(&self, w: &mut impl crate::WriteExt) {
        match self {
            Self::Deploy(deploy) => {
                deploy.encode(w);
            }
            Self::Spawn(spawn) => {
                spawn.encode(w);
            }
            Self::Call(call) => {
                call.encode(w);
            }
        }
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok(match cursor.peek_byte().unwrap() {
            TY_DEPLOY => {
                let receipt = DeployReceipt::decode(cursor)?;
                Receipt::Deploy(receipt)
            }
            TY_SPAWN => {
                let receipt = SpawnReceipt::decode(cursor)?;
                Receipt::Spawn(receipt)
            }
            TY_CALL => {
                let receipt = CallReceipt::decode(cursor)?;
                Receipt::Call(receipt)
            }
            byte => return Err(ParseError::BadByte(byte)),
        })
    }
}

const TY_DEPLOY: u8 = 0;
const TY_SPAWN: u8 = 1;
const TY_CALL: u8 = 2;
