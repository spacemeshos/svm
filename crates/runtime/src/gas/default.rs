use svm_types::{Template, Transaction, SpawnApp};

use super::estimator::{DeployTemplateEst, ExecAppEst, GasEstimator, SpawnAppEst};

/// Default Gas estimation. Implements the `GasEstimator` trait.
pub struct DefaultGasEstimator;

impl GasEstimator for DefaultGasEstimator {
    fn est_deploy_template(_bytes: &[u8], _template: &Template) -> DeployTemplateEst {
        todo!()
    }

    fn est_spawn_app(_bytes: &[u8], _spawn: &SpawnApp) -> SpawnAppEst {
        todo!()
    }

    fn est_exec_app(_bytes: &[u8], _tx: &Transaction) -> ExecAppEst {
        todo!()
    }
}
