use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HostCtx {
    pub(crate) inner: HashMap<i32, Vec<u8>>,
}

impl From<HashMap<i32, Vec<u8>>> for HostCtx {
    fn from(inner: HashMap<i32, Vec<u8>>) -> Self {
        Self { inner }
    }
}

impl HostCtx {
    pub fn into_inner(self) -> HashMap<i32, Vec<u8>> {
        self.inner
    }
}
