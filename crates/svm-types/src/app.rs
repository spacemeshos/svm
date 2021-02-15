use std::fmt;

use crate::TemplateAddr;

/// An in-memory representation of an app.
#[derive(PartialEq)]
pub struct App {
    /// `App`'s name
    pub name: String,

    /// `Address` of the `Template`, the App is being spawned from.
    pub template: TemplateAddr,
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("App")
            .field("name", &self.name)
            .field("template", self.template.inner())
            .finish()
    }
}
