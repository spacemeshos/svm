use std::cmp::min;
use std::fmt;

use svm_layout::DataLayout;

/// An in-memory representation of an app-template.
#[allow(missing_docs)]
#[derive(PartialEq)]
pub struct AppTemplate {
    pub version: u16,
    pub name: String,
    pub code: Vec<u8>,
    pub data: DataLayout,
    pub ctors: Vec<String>,
}

impl fmt::Debug for AppTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AppTemplate")
            .field("version", &self.version)
            .field("name", &self.name)
            .field("code", &fmt_code(&self.code))
            .field("data", &self.data)
            .field("ctors", &self.ctors)
            .finish()
    }
}

fn fmt_code(code: &[u8]) -> String {
    let n = std::cmp::min(code.len(), 4);

    format!("{:?}", &code[0..n])
}
