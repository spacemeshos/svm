use crate::traits::{ContractDeserializer, ContractSerializer};
use serde::{Deserialize, Serialize};

use svm_common::Address;

/// We first parse the on-the-wire contract transaction into a `WasmContract` instance.
/// At that stage we don't know the contract future `address` yet.
///
/// It's only later, while we `validiate` the contract when we also compute its future account address and add it to the `WasmContract` instance.
/// That's the reason why the `Address` field is defined of type `Option<Address>` and not simply `Address`.
#[derive(Serialize, Deserialize)]
pub struct WasmContract {
    pub address: Option<Address>,
    pub wasm: Vec<u8>,
    pub name: String,
    pub author: Address,
}

impl std::fmt::Debug for WasmContract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let addr = self.preview_address(&self.address);
        let author = self.preview_author(&self.author);
        let wasm = self.preview_wasm(&self.wasm);

        let msg = [addr, author, wasm].join("\n");

        write!(f, "{}", msg)
    }
}

impl WasmContract {
    fn preview_address(&self, addr: &Option<Address>) -> String {
        match addr {
            Some(addr) => {
                let slice = &addr.as_slice()[24..31];
                format!("Address: {:?}...", slice)
            }
            None => String::from("Address: None"),
        }
    }

    fn preview_author(&self, author: &Address) -> String {
        format!("Author: {:?}...", &author.as_slice()[0..8])
    }

    fn preview_wasm(&self, wasm: &[u8]) -> String {
        format!("Code: {:?}", &wasm[0..4])
    }
}
