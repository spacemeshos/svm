use crate::traits::GasEstimator;

pub struct DefaultGasEstimator;

impl GasEstimator for DefaultGasEstimator {
    fn estimate_template(bytes: &[u8]) -> u64 {
        todo!()
    }

    fn estimate_app(bytes: &[u8]) -> u64 {
        todo!()
    }

    fn estimate_tx(bytes: &[u8]) -> u64 {
        todo!()
    }
}
