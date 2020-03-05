use std::fmt;

use svm_common::Address;

/// An in-memory representation of an app.
#[derive(PartialEq)]
pub struct App {
    pub version: u32,

    /// `Address` of the `AppTemplate` app is being spawned from.
    pub template: Address,

    pub creator: Address,
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = self.fmt_version(*&self.version);
        let template = self.fmt_template(&self.template);
        let creator = self.fmt_creator(&self.creator);

        let msg = [version, template, creator].join("\n");

        write!(f, "{}", msg)
    }
}

impl App {
    fn fmt_version(&self, ver: u32) -> String {
        format!("Version: {}", ver)
    }

    fn fmt_template(&self, addr: &Address) -> String {
        format!("Template: {}", self.fmt_address(addr))
    }

    fn fmt_creator(&self, addr: &Address) -> String {
        format!("Creator: {}", self.fmt_address(addr))
    }

    fn fmt_address(&self, addr: &Address) -> String {
        addr.fmt(4, 4, " ")
    }
}
