use svm_layout::FixedLayout;
use svm_storage::app::AppStorage;
use svm_types::{AccountAddr, State};

use crate::Config;

/// `AppStorage` building function signature.
pub type StorageBuilderFn = dyn Fn(&AccountAddr, &State, &FixedLayout, &Config) -> AppStorage;
