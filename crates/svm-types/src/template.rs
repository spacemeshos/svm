use std::fmt;

use svm_layout::Layout;

/// An in-memory representation of an app-template.
#[allow(missing_docs)]
#[derive(PartialEq)]
pub struct Template {
    pub version: u16,
    pub name: String,
    pub code: Vec<u8>,
    pub data: Layout,
    pub ctors: Vec<String>,
}

impl fmt::Debug for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Template")
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
