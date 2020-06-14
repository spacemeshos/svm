use std::fmt;

use svm_layout::DataLayout;

/// An in-memory representation of an app-template.
#[allow(missing_docs)]
#[derive(PartialEq)]
pub struct AppTemplate {
    pub version: u32,
    pub name: String,
    pub code: Vec<u8>,
    pub data: DataLayout,
}

impl fmt::Debug for AppTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ver = self.fmt_version(self.version);
        let name = self.fmt_name(&self.name);
        let code = self.fmt_code(&self.code);
        let data = self.fmt_data(&self.data);

        let msg = [ver, name, code, data].join("\n");

        write!(f, "{}", msg)
    }
}

impl AppTemplate {
    fn fmt_version(&self, ver: u32) -> String {
        format!("Version: {}", ver)
    }

    fn fmt_name(&self, name: &str) -> String {
        format!("Name: {}", name)
    }

    fn fmt_code(&self, code: &[u8]) -> String {
        format!("Code: {:?}", &code[0..4])
    }

    fn fmt_data(&self, data: &DataLayout) -> String {
        format!("Data-Layout: {:?}", data)
    }
}
