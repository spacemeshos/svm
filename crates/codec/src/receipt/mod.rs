//! Encoding of receipts.

mod call;
mod deploy;
mod error;
mod spawn;

pub(crate) mod logs;

use svm_types::{CallReceipt, DeployReceipt, Receipt, SpawnReceipt};

use crate::{Codec, ReadExt};

impl Codec for Receipt {
    type Error = std::convert::Infallible;

    fn encode(&self, w: &mut impl crate::WriteExt) {
        match self {
            Self::Deploy(deploy) => {
                w.write_byte(TY_DEPLOY);
                deploy.encode(w);
            }
            Self::Spawn(spawn) => {
                w.write_byte(TY_SPAWN);
                spawn.encode(w);
            }
            Self::Call(call) => {
                w.write_byte(TY_CALL);
                call.encode(w);
            }
        }
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok(match cursor.seek_byte().unwrap() {
            TY_DEPLOY => {
                let receipt = DeployReceipt::decode(cursor).unwrap();
                Receipt::Deploy(receipt)
            }
            TY_SPAWN => {
                let receipt = SpawnReceipt::decode(cursor).unwrap();
                Receipt::Spawn(receipt)
            }
            TY_CALL => {
                let receipt = CallReceipt::decode(cursor).unwrap();
                Receipt::Call(receipt)
            }
            _ => unreachable!(),
        })
    }
}

const TY_DEPLOY: u8 = 0;
const TY_SPAWN: u8 = 1;
const TY_CALL: u8 = 2;
