use crate::types::CodeHash;
use crate::wasm::Contract;

use svm_common::Address;

/// Serializing a contract into its raw representation trait.
pub trait ContractSerializer {
    #[allow(missing_docs)]
    fn serialize(contract: &Contract) -> Vec<u8>;
}

/// Deserializing raw contract into its in-memory representation trait.
pub trait ContractDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: Vec<u8>) -> Contract;
}

/// Stores serialized contracts (a.k.a raw contracts)
/// and deserializes raw contract into `Contract` upon fetching.
pub trait ContractStore<S, D>
where
    S: ContractSerializer,
    D: ContractDeserializer,
{
    /// Stores the `hash` -> `raw contract` association
    fn store(&mut self, contract: &Contract, hash: CodeHash);

    /// Given a contract account address, fetches its raw contract dada
    /// and deserializes it. Return `None` it contract doesn't exist
    fn load(&self, address: Address) -> Option<Contract>;
}

/// Computes a contract account address.
/// Algorithm must be deterministic.
pub trait ContractAddressCompute {
    fn compute(contract: &Contract) -> Address;
}

/// Computes code-hash derived deterministically from raw contract.
pub trait ContractCodeHasher {
    fn hash(bytes: &[u8]) -> CodeHash;
}
