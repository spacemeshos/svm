use svm_types::{Gas, SpawnApp, Template, Transaction};

/// Holds estimated gas for deploying a new `Template`.
pub struct DeployTemplateEst {
    pub payload: Gas,

    pub install: Gas,
}

/// Holds estimated gas for spawning an new `App`.
pub struct SpawnAppEst {
    pub payload: Gas,

    pub install: Gas,

    pub ctor: Gas,
}

/// Holds estimated gas for executing an `AppTransaction`.
pub struct ExecAppEst {
    pub payload: Gas,

    pub exec: Gas,
}

/// Trait in charge on doing gas estimation.
pub trait GasEstimator {
    /// Estimates the gas required for deploying `template`. (`bytes` is the deploy-template raw format).
    fn est_deploy_template(bytes: &[u8], template: &Template) -> DeployTemplateEst;

    /// Estimates the gas required for spawning app `spawn`. (`bytes` is the spawn-app raw format).
    fn est_spawn_app(bytes: &[u8], spawn: &SpawnApp) -> SpawnAppEst;

    /// Estimates the gas required for executing transaction `tx`. (`bytes` is the exec-app raw format).
    fn est_exec_app(bytes: &[u8], tx: &Transaction) -> ExecAppEst;
}
