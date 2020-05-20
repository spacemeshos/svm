use std::path::{Path, PathBuf};

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// The path for the key-value estore
    pub kv_path: PathBuf,
}

impl Config {
    pub fn new<P: AsRef<Path>>(kv_path: P) -> Self {
        Self {
            kv_path: kv_path.as_ref().to_path_buf(),
        }
    }
}
