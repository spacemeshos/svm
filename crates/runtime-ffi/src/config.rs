use lazy_static::lazy_static;

use std::path::PathBuf;
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Mutex::default();
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db_path: Option<PathBuf>,
}

impl Config {
    /// Initializes the global [`Config`] instance.
    pub fn set(config: Config) {
        *CONFIG.lock().unwrap() = Some(config);
    }

    /// Returns `true` if and only if the global [`Config`] instance has been
    /// initialized via [`Config::set`].
    pub fn is_ready() -> bool {
        CONFIG.lock().unwrap().is_some()
    }

    /// Fetches the global [`Config`] instance.
    ///
    /// # Panics
    ///
    /// Panics if the global [`Config`] hasn't been initialized with
    /// [`Config::set`].
    pub fn get() -> Config {
        CONFIG.lock().unwrap().as_ref().unwrap().clone()
    }
}
