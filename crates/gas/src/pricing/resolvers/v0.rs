use crate::{Op, PriceResolver};

pub struct V0Resolver;

impl Default for V0Resolver {
    fn default() -> Self {
        V0Resolver {}
    }
}

impl PriceResolver for V0Resolver {
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
