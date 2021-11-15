use svm_sdk::{template, Address};

#[template]
mod GenesisTemplate {
    #[ctor]
    fn initialize() -> bool {
        true
    }

    #[endpoint]
    fn transfer(addr: Address, amount: u64) {
        svm_transfer(addr, amount);
    }
}
