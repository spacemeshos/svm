pub use crate::traits::ContractAddressCompute;

use serde::{Deserialize, Serialize};

/// Represents a contract code hash
#[repr(transparent)]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct CodeHash(pub [u8; 32]);
