pub type CodeHash = [u8; 32];

/// This trait defines the interface for storing / retrieving a unit of code.
///
/// A unit of code may be:
/// * A Program - a smart contract program
/// * A Package - packages used by the smart contract
///
/// Since we want to encourage code reuse between smart contracts,
/// we'd want to enforce storing each code package only once.
pub trait CodeRepository {
    fn exists(&self, code_hash: &CodeHash) -> bool;

    fn try_get(&self, code_hash: &CodeHash) -> Option<Vec<u8>>;

    fn store<H: hash_db::Hasher<Out = CodeHash>>(&mut self, code: &[u8]) -> CodeHash;
}
