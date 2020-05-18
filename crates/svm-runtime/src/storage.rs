use svm_app::types::AppAddr;
use svm_common::State;

use svm_storage::AppStorage;

use svm_storage2::app::AppStorage as AppStorage2;

use crate::settings::AppSettings;

/// Represents a function that builds a `AppStorage` given its address, state and settings.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &AppSettings) -> AppStorage;

/// Represents a function that builds a `AppStorage2` given its address, state and settings.
pub type Storage2BuilderFn = dyn Fn(&AppAddr, &State, &AppSettings) -> AppStorage2;
