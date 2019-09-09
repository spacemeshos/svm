use crate::traits::ContractAddressCompute;
use crate::wasm::Contract;
use svm_common::{Address, DefaultKeyHasher, KeyHasher};

/// Default implementation for `ContractAddressCompute`.
///
/// Computing the contract's account address as follows:
/// Taking `Address::len()` bytes of `HASH(contract.author || contract.wasm)`
pub struct DefaultContractAddressCompute;

impl ContractAddressCompute for DefaultContractAddressCompute {
    fn compute(contract: &Contract) -> Address {
        let mut buf = Vec::with_capacity(Address::len() + contract.wasm.len());
        buf.extend_from_slice(contract.author.as_slice());
        buf.extend_from_slice(contract.wasm.as_slice());

        let hash = DefaultKeyHasher::hash(&buf);

        Address::from(&hash[0..Address::len()])
    }
}
