use svm_layout::FixedLayout;
use svm_storage::app::AppStorage;
use svm_types::{AppAddr, State};

use crate::Config;

/// `AppStorage` building function signature.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &FixedLayout, &Config) -> AppStorage;
