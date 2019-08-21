use svm_common::Address;

/// We first parse the on-the-wire contract transaction into a `WasmContract` instance.
/// At that stage we don't know the contract future `address` yet.
///
/// It's only later, while we `validiate` the contract when we also compute its future account address and add it to the `WasmContract` instance.
/// That's the reason why the `address` field is defined as `Option<Address>` and not simply `Address`.
pub struct WasmContract {
    pub Address: Option<Address>,
    pub Wasm: Vec<u8>,
    pub Name: String,
    pub Author: Address,
    pub Admins: Vec<Address>,
}

impl std::fmt::Debug for WasmContract {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let addr = self.preview_address(&self.Address);
        let author = self.preview_author(&self.Author);
        let wasm = self.preview_wasm(&self.Wasm);

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
