use svm_common::Address;

/// We first parse the on-the-wire contract transaction into a `WasmContract` instance.
/// At that stage we don't know the contract future `address` yet.
///
/// It's only later, while we `validiate` the contract when we also compute it's future account address and add it to the `WasmContract` instance.
/// That's the reason why the `address` field is defined as `Option<Address>` and not simply `Address`.
pub struct WasmContract {
    pub(crate) address: Option<Address>,
    pub(crate) name: String,
    pub(crate) wasm: Vec<u8>,
    pub(crate) author: Address,
    pub(crate) admins: Vec<Address>,
}
