use std::fmt;

use serde::{Deserialize, Serialize};
use svm_common::Address;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct App {
    pub template: Address,
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
