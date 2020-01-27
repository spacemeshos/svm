use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct HostCtx {
    pub(crate) inner: HashMap<u32, Vec<u8>>,
}

impl From<HashMap<u32, Vec<u8>>> for HostCtx {
    fn from(inner: HashMap<u32, Vec<u8>>) -> Self {
        Self { inner }
    }
}

impl HostCtx {
    pub fn into_inner(self) -> HashMap<u32, Vec<u8>> {
        self.inner
    }
}
