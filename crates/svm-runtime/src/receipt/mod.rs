mod deploy_template;
mod exec_app;
mod spawn_app;

use crate::value::Value;

pub use deploy_template::TemplateReceipt;
pub use exec_app::ExecReceipt;
pub use spawn_app::{make_spawn_app_receipt, SpawnAppReceipt};

pub enum Receipt<'a> {
    DeployTemplate(&'a TemplateReceipt),
    SpawnApp(&'a SpawnAppReceipt),
    ExecApp(&'a ExecReceipt),
}

impl<'a> Receipt<'a> {
    pub fn is_success(&self) -> bool {
        match self {
            Self::DeployTemplate(r) => r.success,
            Self::SpawnApp(r) => r.success,
            Self::ExecApp(r) => r.success,
        }
    }

    pub fn gas_used(&self) -> u64 {
        match self {
            Self::DeployTemplate(r) => r.gas_used.unwrap(),
            Self::SpawnApp(r) => r.gas_used.unwrap(),
            Self::ExecApp(r) => r.gas_used.unwrap(),
        }
    }

    pub fn get_returns(&self) -> &Vec<Value> {
        match self {
            Self::DeployTemplate(..) => unreachable!(),
            Self::SpawnApp(r) => r.get_returns(),
            Self::ExecApp(r) => r.get_returns(),
        }
    }

    pub fn error_string(&self) -> String {
        match self {
            Self::DeployTemplate(r) => r.error.as_ref().unwrap().to_string(),
            Self::SpawnApp(r) => r.error.as_ref().unwrap().to_string(),
            Self::ExecApp(r) => r.error.as_ref().unwrap().to_string(),
        }
    }
}
