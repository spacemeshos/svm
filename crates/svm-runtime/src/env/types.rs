/// Represents an `Template` Hash
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[repr(transparent)]
pub struct TemplateHash(pub [u8; 32]);
