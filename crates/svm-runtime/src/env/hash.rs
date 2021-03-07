/// Represents an `Template` Hash
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[repr(transparent)]
pub struct TemplateHash([u8; TemplateHash::len()]);

impl TemplateHash {
    pub fn new(bytes: [u8; TemplateHash::len()]) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub const fn len() -> usize {
        32
    }
}
