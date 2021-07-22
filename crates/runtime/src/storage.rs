use svm_layout::FixedLayout;
use svm_storage::account::AccountStorage;
use svm_types::{AccountAddr, State};

use crate::Config;

/// [`AccountStorage`] building function signature.
pub type StorageBuilderFn = dyn Fn(&AccountAddr, &State, &FixedLayout, &Config) -> AccountStorage;
