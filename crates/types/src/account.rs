use std::fmt;

use crate::TemplateAddr;

/// An in-memory representation of an [`Account`].
#[derive(PartialEq, Clone)]
pub struct Account {
    /// [`Account`]'s name
    pub name: String,

    /// Address of the `Template`, the [`Account`] is being spawned from.
    pub template_addr: TemplateAddr,
}

#[doc(hidden)]
impl Account {
    pub fn new(template_addr: TemplateAddr, name: String) -> Self {
        Self {
            name,
            template_addr,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        &self.template_addr
    }
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Account")
            .field("name", &self.name())
            .field("template", self.template_addr().inner())
            .finish()
    }
}
