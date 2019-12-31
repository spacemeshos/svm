pub use crate::traits::AppTemplateAddressCompute;

use serde::{Deserialize, Serialize};

/// Represents an `AppTemplate` code hash
#[repr(transparent)]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct CodeHash(pub [u8; 32]);
