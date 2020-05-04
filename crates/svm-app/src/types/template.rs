use std::fmt;

use svm_storage2::layout::DataLayout;

/// An in-memory representation of an app-template.
#[allow(missing_docs)]
#[derive(PartialEq)]
pub struct AppTemplate {
    pub version: u32,
    pub name: String,
    pub page_count: u16,
    pub code: Vec<u8>,
    pub data: DataLayout,
}

impl fmt::Debug for AppTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ver = self.fmt_version(self.version);
        let name = self.fmt_name(&self.name);
        let pages = self.fmt_page_count(self.page_count);
        let code = self.fmt_code(&self.code);

        let msg = [ver, name, code, pages].join("\n");

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

    fn fmt_page_count(&self, page_count: u16) -> String {
        format!("#Pages: {}", page_count)
    }

    fn fmt_code(&self, code: &[u8]) -> String {
        format!("Code: {:?}", &code[0..4])
    }
}
