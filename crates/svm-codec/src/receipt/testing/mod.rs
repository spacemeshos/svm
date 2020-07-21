mod receipt;

pub use receipt::{ClientAppReceipt, ClientExecReceipt, ClientTemplateReceipt};

use receipt::{decode_app_receipt, decode_exec_receipt, decode_template_receipt};

use super::types;

#[derive(Debug, PartialEq)]
pub enum ClientReceipt {
    DeployTemplate(ClientTemplateReceipt),

    SpawnApp(ClientAppReceipt),

    ExecApp(ClientExecReceipt),
}

impl ClientReceipt {
    pub fn into_deploy_template(self) -> ClientTemplateReceipt {
        match self {
            ClientReceipt::DeployTemplate(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn into_spawn_app(self) -> ClientAppReceipt {
        match self {
            ClientReceipt::SpawnApp(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn into_exec_app(self) -> ClientExecReceipt {
        match self {
            ClientReceipt::ExecApp(r) => r,
            _ => unreachable!(),
        }
    }
}

pub fn decode_receipt(bytes: &[u8]) -> ClientReceipt {
    assert!(bytes.len() > 0);

    let ty = bytes[0];

    match ty {
        types::DEPLOY_TEMPLATE => {
            let receipt = decode_template_receipt(bytes);
            ClientReceipt::DeployTemplate(receipt)
        }
        types::SPAWN_APP => {
            let receipt = decode_app_receipt(bytes);
            ClientReceipt::SpawnApp(receipt)
        }
        types::EXEC_APP => {
            let receipt = decode_exec_receipt(bytes);
            ClientReceipt::ExecApp(receipt)
        }
        _ => unreachable!(),
    }
}
