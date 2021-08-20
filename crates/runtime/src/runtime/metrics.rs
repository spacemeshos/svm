use prometheus::{Encoder, IntCounter, Registry, TextEncoder};

pub struct Metrics {
    registry: Registry,
    metric_validates: IntCounter,
    metric_validate_errs: IntCounter,
    metric_capi_calls: IntCounter,
}

impl Metrics {
    pub fn validates(&self) -> &IntCounter {
        &self.metric_validates
    }

    pub fn validate_errs(&self) -> &IntCounter {
        &self.metric_validate_errs
    }

    pub fn capi_calls(&self) -> &IntCounter {
        &self.metric_capi_calls
    }

    pub fn export_text(&self) -> Vec<u8> {
        let encoder = TextEncoder::default();
        let mut bytes: Vec<u8> = Vec::new();
        encoder.encode(&self.registry.gather(), &mut bytes).unwrap();
        bytes
    }
}

impl Default for Metrics {
    fn default() -> Self {
        let metric_validates =
            IntCounter::new("validates", "Number of successful transaction validations.").unwrap();
        let metric_validate_errs = IntCounter::new(
            "validates_errs",
            "Number of errors during transaction validation.",
        )
        .unwrap();
        let metric_capi_calls =
            IntCounter::new("capi_calls", "Total number of SVM C API calls.").unwrap();

        let registry = Registry::new();
        registry
            .register(Box::new(metric_validates.clone()))
            .unwrap();
        registry
            .register(Box::new(metric_validate_errs.clone()))
            .unwrap();
        registry
            .register(Box::new(metric_capi_calls.clone()))
            .unwrap();

        Self {
            registry,
            metric_validates,
            metric_validate_errs,
            metric_capi_calls,
        }
    }
}
