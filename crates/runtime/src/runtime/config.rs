use std::path::PathBuf;

/// Runtime configuration
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// The path for the key-value store
    pub kv_path: PathBuf,
}
