use std::path::PathBuf;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// The path for the key-value store
    kv_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kv_path: PathBuf::new(),
        }
    }
}
