use crate::traits::{CodeHashStore, ContractAddressCompute};

pub trait ContractEnv {
    type AddressCompute: ContractAddressCompute;

    type Store: CodeHashStore;

    fn init_store<F: Fn() -> Self::Store>(&mut self, f: F);

    fn get_store(&mut self) -> &Self::Store;

    fn get_store_mut(&mut self) -> &mut Self::Store;

    fn open_store(&mut self);

    fn close_store(&mut self);
}
