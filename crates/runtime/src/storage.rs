use svm_layout::Layout;
use svm_storage::app::AppStorage;
use svm_types::{AppAddr, State};

use crate::Config;

/// `AppStorage` building function signature.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &Layout, &Config) -> AppStorage;
