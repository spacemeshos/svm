use std::fmt;

use crate::{Address, TemplateAddr};

/// An in-memory representation of an app.
#[derive(PartialEq)]
pub struct App {
    /// `App` version.
    pub version: u32,

    /// `Address` of the `AppTemplate`, the App is being spawned from.
    pub template: TemplateAddr,
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = self.fmt_version(self.version);
        let template = self.fmt_template(&self.template);

        let msg = [version, template].join("\n");

        writeln!(f, "{}", msg)
    }
}

impl App {
    fn fmt_version(&self, ver: u32) -> String {
        format!("Version: {}", ver)
    }

    fn fmt_template(&self, addr: &TemplateAddr) -> String {
        format!("Template: {}", self.fmt_address(addr.inner()))
    }

    fn fmt_address(&self, addr: &Address) -> String {
        addr.fmt(4, 4, " ")
    }
}
