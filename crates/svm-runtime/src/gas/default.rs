use svm_app::types::{AppTemplate, AppTransaction, SpawnApp};
use svm_gas::Gas;

use super::estimator::{DeployTemplateEst, ExecAppEst, GasEstimator, SpawnAppEst};

/// Default Gas estimation. Implements the `GasEstimator` trait.
pub struct DefaultGasEstimator;

impl GasEstimator for DefaultGasEstimator {
    fn est_deploy_template(_bytes: &[u8], _template: &AppTemplate) -> DeployTemplateEst {
        todo!()
    }

    fn est_spawn_app(_bytes: &[u8], _spawn: &SpawnApp) -> SpawnAppEst {
        todo!()
    }

    fn est_exec_app(_bytes: &[u8], _tx: &AppTransaction) -> ExecAppEst {
        todo!()
    }
}

impl DefaultGasEstimator {
    #[inline]
    pub fn payload_gas(bytes: &[u8]) -> Gas {
        let payload_size = bytes.len() as u64;
        Gas::Fixed(payload_size * 10_000)
    }
}
