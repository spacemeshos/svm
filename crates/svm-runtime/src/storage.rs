use svm_app::types::AppAddr;
use svm_common::State;
use svm_storage::app::AppStorage;

use crate::settings::AppSettings;

/// Represents a function that builds a `AppStorage` given its address, state and settings.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &AppSettings) -> AppStorage;
