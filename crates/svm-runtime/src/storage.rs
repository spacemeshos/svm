use svm_common::State;
use svm_layout::DataLayout;
use svm_storage::app::AppStorage;
use svm_types::AppAddr;

use crate::Config;

/// `AppStorage` building function signature.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &DataLayout, &Config) -> AppStorage;
