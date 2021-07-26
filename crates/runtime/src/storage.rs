use svm_layout::FixedLayout;
use svm_storage::account::AccountStorage;
use svm_types::{Address, State};

use crate::Config;

/// [`AccountStorage`] building function signature.
pub type StorageBuilderFn = dyn Fn(&Address, &State, &FixedLayout, &Config) -> AccountStorage;
