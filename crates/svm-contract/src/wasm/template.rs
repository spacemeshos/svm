use serde::{Deserialize, Serialize};

use svm_common::Address;

/// We first parse the on-the-wire `app-template` deploy transaction into a `AppTemplate` instance.
/// At that stage we don't know the `app-template` future `Address` yet.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct AppTemplate {
    pub code: Vec<u8>,
    pub name: String,
    pub author: Address,
}

impl std::fmt::Debug for AppTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let author = self.fmt_author(&self.author);
        let code = self.fmt_code(&self.code);

        let msg = [author, code].join("\n");

        write!(f, "{}", msg)
    }
}

impl AppTemplate {
    fn fmt_author(&self, author: &Address) -> String {
        format!("Author: {:?}...", &author.as_slice()[0..8])
    }

    fn fmt_code(&self, code: &[u8]) -> String {
        format!("Code: {:?}", &code[0..4])
    }
}
