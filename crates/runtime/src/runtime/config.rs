use std::path::PathBuf;

/// Runtime configuration
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// The path for the key-value store
    kv_path: PathBuf,
}
