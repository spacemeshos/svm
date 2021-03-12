mod deploy_template;
mod error;
mod exec_app;
mod gas;
mod spawn_app;

pub(crate) mod logs;

pub(crate) use error::{decode_error, encode_error};

mod types {
    pub const DEPLOY_TEMPLATE: u8 = 0;

    pub const SPAWN_APP: u8 = 1;

    pub const EXEC_APP: u8 = 2;
}

pub use deploy_template::{decode_template_receipt, encode_template_receipt};
pub use exec_app::{decode_exec_receipt, encode_exec_receipt};
pub use spawn_app::{decode_app_receipt, encode_app_receipt};

use svm_types::Receipt;

/// Decodes a binary Receipt into its Rust struct wrapped as `ReceiptOwned`
pub fn decode_receipt(bytes: &[u8]) -> Receipt {
    assert!(bytes.len() > 0);

    let ty = bytes[0];

    match ty {
        types::DEPLOY_TEMPLATE => {
            let receipt = decode_template_receipt(bytes);
            Receipt::DeployTemplate(receipt)
        }
        types::SPAWN_APP => {
            let receipt = decode_app_receipt(bytes);
            Receipt::SpawnApp(receipt)
        }
        types::EXEC_APP => {
            let receipt = decode_exec_receipt(bytes);
            Receipt::ExecApp(receipt)
        }
        _ => unreachable!(),
    }
}
