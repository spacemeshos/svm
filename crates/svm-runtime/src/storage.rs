use svm_app::types::AppAddr;
use svm_common::State;
use svm_layout::DataLayout;
use svm_storage::app::AppStorage;

use crate::Config;

/// `AppStorage` building function signature.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &DataLayout, &Config) -> AppStorage;
