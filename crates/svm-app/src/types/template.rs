use std::fmt;

use serde::{Deserialize, Serialize};

use svm_common::Address;

/// We first parse the raw `app-template` bytes into an `AppTemplate` instance.
/// At that stage we don't know the `app-template` future `Address` yet.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct AppTemplate {
    pub name: String,
    pub author: Address,
    pub pages_count: u16,
    pub code: Vec<u8>,
}

impl fmt::Debug for AppTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.fmt_name(&self.name);
        let author = self.fmt_author(&self.author);
        let pages = self.fmt_pages_count(self.pages_count);
        let code = self.fmt_code(&self.code);

        let msg = [author, name, code, pages].join("\n");
        write!(f, "{}", msg)
    }
}

impl AppTemplate {
    fn fmt_name(&self, name: &str) -> String {
        format!("Name: {:?}", name)
    }

    fn fmt_author(&self, author: &Address) -> String {
        format!("Author: {:?}...", &author.as_slice()[0..8])
    }

    fn fmt_pages_count(&self, pages_count: u16) -> String {
        format!("#Pages: {:?}", pages_count)
    }

    fn fmt_code(&self, code: &[u8]) -> String {
        format!("Code: {:?}", &code[0..4])
    }
}
