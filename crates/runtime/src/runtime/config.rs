use std::path::PathBuf;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// The path for the key-value store
    pub kv_path: PathBuf,
}
