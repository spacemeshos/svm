use crate::{impl_from_svm_byte_array, impl_into_svm_byte_array};

use svm_common::State;

impl_from_svm_byte_array!(State);
impl_into_svm_byte_array!(State);
