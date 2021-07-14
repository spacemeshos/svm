mod call;
mod deploy;
mod error;
mod gas;
mod spawn;

pub(crate) mod logs;

pub(crate) use error::{decode_error, encode_error};

pub use call::{decode_call, encode_call};
pub use deploy::{decode_deploy, encode_deploy_receipt};
pub use spawn::{decode_spawn, encode_spawn};

use svm_types::Receipt;

mod types {
    pub const DEPLOY: u8 = 0;
    pub const SPAWN: u8 = 1;
    pub const CALL: u8 = 2;
}

/// Decodes a binary Receipt into its Rust struct wrapped as `ReceiptOwned`
pub fn decode_receipt(bytes: &[u8]) -> Receipt {
    assert!(bytes.len() > 0);

    let ty = bytes[0];

    match ty {
        types::DEPLOY => {
            let receipt = decode_deploy(bytes);
            Receipt::Deploy(receipt)
        }
        types::SPAWN => {
            let receipt = decode_spawn(bytes);
            Receipt::Spawn(receipt)
        }
        types::CALL => {
            let receipt = decode_call(bytes);
            Receipt::Call(receipt)
        }
        _ => unreachable!(),
    }
}
