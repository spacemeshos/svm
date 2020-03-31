use std::path::PathBuf;

/// Holds settings for using the Runtime.
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// #pages required by the app storage
    pub page_count: u16,

    /// The path for the kv store
    pub kv_path: PathBuf,

    /// Whether gas metering is enabled / disabled.
    pub gas_metering: bool,
}
