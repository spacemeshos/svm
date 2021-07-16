use svm_program::Op;

use crate::PriceResolver;

/// An example `PriceResolver`
#[derive(Copy, Clone, Debug)]
pub struct ExampleResolver;

impl Default for ExampleResolver {
    fn default() -> Self {
        ExampleResolver {}
    }
}

impl PriceResolver for ExampleResolver {
    fn op_price(&self, op: &Op) -> usize {
        1
    }

    fn import_price(&self, import: (&str, &str)) -> usize {
        match import {
            ("svm", "svm_static_alloc") => 100,
            ("svm", "svm_calldata_offset") => 10,
            ("svm", "svm_calldata_len") => 10,
            ("svm", "svm_set_returndata") => 20,
            ("svm", "svm_get32") => 100,
            ("svm", "svm_set32") => 1_000_000,
            ("svm", "svm_get64") => 200,
            ("svm", "svm_set64") => 2_000_000,
            ("svm", "svm_load160") => 500,
            ("svm", "svm_store160") => 5_000_000,
            ("svm", "svm_log") => 3_000,
            _ => unreachable!(),
        }
    }
}
