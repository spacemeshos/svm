use crate::{Section, SectionKind};

/// Contains descriptive info about a `Template`
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderSection {
    name: String,

    desc: String,

    code_version: u32,
}

impl HeaderSection {
    /// Creates a new Section
    pub fn new(name: String, desc: String, code_version: u32) -> Self {
        Self {
            code_version,
            name,
            desc,
        }
    }

    /// Borrows the `Name` of the `Template`
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Borrows the `Description` of the `Template`
    pub fn desc(&self) -> &str {
        &self.desc
    }

    /// Borrows the `Code Version` of the `Template`
    pub fn code_version(&self) -> u32 {
        self.code_version
    }
}

impl Section for HeaderSection {
    const KIND: SectionKind = SectionKind::Header;
}
