/// Holds settings for using the runtime.
#[derive(Debug, Clone, Copy)]
pub struct Opts {
    /// maximum pages required by the contract pages storage
    pub max_pages: usize,
}
