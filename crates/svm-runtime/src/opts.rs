/// Holds settings for using the runtime.
#[derive(Debug, Clone)]
pub struct Opts {
    /// maximum pages required by the contract pages storage
    pub max_pages: usize,

    /// kv path
    pub kv_path: String,
}
