use svm_app::types::{AppTemplate, AppTransaction, SpawnApp};

pub struct DeployTemplateEst {
    pub payload: u64,

    pub install: u64,
}

pub struct SpawnAppEst {
    pub payload: u64,

    pub install: u64,

    pub ctor: u64,
}

pub struct ExecAppEst {
    pub payload: u64,

    pub exec: u64,
}

pub trait GasEstimator {
    fn est_deploy_template(bytes: &[u8], template: &AppTemplate) -> DeployTemplateEst;

    fn est_spawn_app(bytes: &[u8], spawn: &SpawnApp) -> SpawnAppEst;

    fn est_exec_app(bytes: &[u8], tx: &AppTransaction) -> ExecAppEst;
}
