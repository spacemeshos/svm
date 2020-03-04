use std::fmt;

use svm_common::Address;

/// An in-memory representation of an app.
#[derive(PartialEq)]
pub struct App {
    /// `Address` of the `AppTemplate` app is being spawned from.
    pub template: Address,

    /// `Address` of app creator
    pub creator: Address,
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template = self.fmt_template(&self.template);
        let creator = self.fmt_creator(&self.creator);

        let msg = [template, creator].join("\n");
        write!(f, "{}", msg)
    }
}

impl App {
    #[inline]
    fn fmt_template(&self, addr: &Address) -> String {
        format!("Template: {}", self.fmt_address(addr))
    }

    #[inline]
    fn fmt_creator(&self, addr: &Address) -> String {
        format!("Creator: {}", self.fmt_address(addr))
    }

    #[inline]
    fn fmt_address(&self, addr: &Address) -> String {
        addr.fmt(4, 4, " ")
    }
}
