mod deploy;
mod error;
mod call;
mod gas;
mod spawn;

pub(crate) mod logs;

pub(crate) use error::{decode_error, encode_error};

pub use deploy::{decode_deploy_receipt, encode_deploy_receipt};
pub use call::{decode_exec_receipt, encode_call_receipt};
pub use spawn::{decode_app_receipt, encode_app_receipt};

use svm_types::Receipt;

mod types {
    pub const DEPLOY_TEMPLATE: u8 = 0;
    pub const SPAWN_APP: u8 = 1;
    pub const EXEC_APP: u8 = 2;
}

/// Decodes a binary Receipt into its Rust struct wrapped as `ReceiptOwned`
pub fn decode_receipt(bytes: &[u8]) -> Receipt {
    assert!(bytes.len() > 0);

    let ty = bytes[0];

    match ty {
        types::DEPLOY_TEMPLATE => {
            let receipt = decode_deploy_receipt(bytes);
            Receipt::Deploy(receipt)
        }
        types::SPAWN_APP => {
            let receipt = decode_app_receipt(bytes);
            Receipt::Spawn(receipt)
        }
        types::EXEC_APP => {
            let receipt = decode_exec_receipt(bytes);
            Receipt::Call(receipt)
        }
        _ => unreachable!(),
    }
}
