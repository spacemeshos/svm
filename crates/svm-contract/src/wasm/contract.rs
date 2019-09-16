use serde::{Deserialize, Serialize};

use svm_common::Address;

/// We first parse the on-the-wire contract transaction into a `Contract` instance.
/// At that stage we don't know the contract future `address` yet.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Contract {
    pub wasm: Vec<u8>,
    pub name: String,
    pub author: Address,
}

impl std::fmt::Debug for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let author = self.preview_author(&self.author);
        let wasm = self.preview_wasm(&self.wasm);

        let msg = [author, wasm].join("\n");

        write!(f, "{}", msg)
    }
}

impl Contract {
    fn preview_author(&self, author: &Address) -> String {
        format!("Author: {:?}...", &author.as_slice()[0..8])
    }

    fn preview_wasm(&self, wasm: &[u8]) -> String {
        format!("Code: {:?}", &wasm[0..4])
    }
}
