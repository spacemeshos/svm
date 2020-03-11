pub use crate::traits::AppTemplateAddressCompute;

/// Represents an `AppTemplate` Hash
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
#[repr(transparent)]
pub struct AppTemplateHash(pub [u8; 32]);
