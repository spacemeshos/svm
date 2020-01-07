pub use crate::traits::AppTemplateAddressCompute;

use serde::{Deserialize, Serialize};

/// Represents an `AppTemplate` Hash
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Debug)]
pub struct AppTemplateHash(pub [u8; 32]);
