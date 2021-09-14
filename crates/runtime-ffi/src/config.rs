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
    pub fn set(config: Config) {
        *CONFIG.lock().unwrap() = Some(config);
    }

    pub fn is_ready() -> bool {
        CONFIG.lock().unwrap().is_some()
    }

    pub fn get() -> Config {
        CONFIG.lock().unwrap().as_ref().unwrap().clone()
    }
}