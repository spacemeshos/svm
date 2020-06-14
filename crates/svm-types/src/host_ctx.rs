use std::collections::HashMap;

/// Holds the `Host Context`.
/// These are fields injected from the Host into the running App.
#[derive(Debug, Clone, PartialEq)]
pub struct HostCtx {
    pub(crate) inner: HashMap<u32, Vec<u8>>,
}

impl HostCtx {
    /// Creates a new `HostCtx` struct.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl From<HashMap<u32, Vec<u8>>> for HostCtx {
    /// Creates a new `HostCtx` from the given `HashMap`.
    fn from(inner: HashMap<u32, Vec<u8>>) -> Self {
        Self { inner }
    }
}

impl HostCtx {
    /// Returns the underlying `HashMap`
    pub fn into_inner(self) -> HashMap<u32, Vec<u8>> {
        self.inner
    }

    /// Looks up field data by its index `field`.
    #[inline]
    pub fn get(&self, field: u32) -> Option<&Vec<u8>> {
        self.inner.get(&field)
    }
}
