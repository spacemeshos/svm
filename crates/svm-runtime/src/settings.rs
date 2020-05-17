use std::path::PathBuf;
use svm_storage2::layout::DataLayout;

/// Holds settings for using the Runtime.
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// #pages required by the app storage
    pub page_count: u16,

    pub layout: DataLayout,

    /// The path for the kv store
    pub kv_path: PathBuf,
}
