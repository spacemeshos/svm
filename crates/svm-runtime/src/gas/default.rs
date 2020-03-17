use svm_app::types::{AppTemplate, AppTransaction, SpawnApp};

use super::estimator::{DeployTemplateEst, ExecAppEst, GasEstimator, SpawnAppEst};

pub struct DefaultGasEstimator;

impl GasEstimator for DefaultGasEstimator {
    fn est_deploy_template(bytes: &[u8], template: &AppTemplate) -> DeployTemplateEst {
        todo!()
    }

    fn est_spawn_app(bytes: &[u8], spawn: &SpawnApp) -> SpawnAppEst {
        todo!()
    }

    fn est_exec_app(bytes: &[u8], tx: &AppTransaction) -> ExecAppEst {
        todo!()
    }
}
