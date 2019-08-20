pub use crate::traits::{CodeHashStore, ContractAddressCompute};

use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct CodeHash(pub [u8; 32]);

pub trait ContractTypes {
    type Store: CodeHashStore;

    type AddressCompute: ContractAddressCompute;
}
