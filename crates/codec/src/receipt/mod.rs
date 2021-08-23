mod call;
mod deploy;
mod error;
mod gas;
mod returndata;
mod spawn;

pub(crate) mod logs;

pub(crate) use error::{decode_error, encode_error};

use svm_types::{CallReceipt, DeployReceipt, Receipt, SpawnReceipt};

use crate::Codec;

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

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let bytes = *cursor.get_ref();
        assert!(bytes.len() > 0);

        Ok(match bytes[0] {
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
