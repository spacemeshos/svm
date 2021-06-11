use crate::AuthorAddr;

#[derive(Clone)]
pub struct DeploySection {
    tx_id: String,

    layer: u64,

    author: AuthorAddr,
}

impl DeploySection {
    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn layer(&self) -> u64 {
        self.layer
    }

    pub fn author(&self) -> &AuthorAddr {
        &self.author
    }
}
