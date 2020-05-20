use std::path::PathBuf;

use svm_layout::DataLayout;

/// Holds settings for using the Runtime.
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// App's data-layout in-memory representation.
    pub layout: DataLayout,

    /// The path for the kv store
    pub kv_path: PathBuf,
}
