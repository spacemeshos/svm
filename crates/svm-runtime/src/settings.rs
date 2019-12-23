/// Holds settings for using the Runtime.
#[derive(Debug, Clone)]
pub struct ContractSettings {
    /// number of pages required by the contract storage
    pub pages_count: u32,

    /// kv path
    pub kv_path: String,
}
